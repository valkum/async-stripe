//! Deduplicate structurally identical fieldless enums across every component.
//!
//! The spec repeats the same inline enum in many places (e.g. `setup_future_usage` appears
//! under every payment method option of every request). Each copy used to be generated as
//! its own type with its own `as_str`, `FromStr`, `Debug`, serde and miniserde impls. This
//! pass emits one canonical copy in `stripe_shared` and leaves a `pub type` alias with the
//! old name at each site, so the public API keeps every existing name.
use heck::ToSnakeCase;
use indexmap::{IndexMap, IndexSet};
use tracing::info;

use crate::components::{Components, TypeSpec};
use crate::deduplication::infer_dedupped_object_for;
use crate::rust_object::{ObjectKind, ObjectMetadata, ObjectUsage, RustObject};
use crate::rust_type::{PathToType, RustType};
use crate::stripe_object::TypeAlias;
use crate::types::RustIdent;
use crate::visitor::{Visit, VisitMut};

#[derive(Default)]
struct CollectFieldlessEnums {
    objs: IndexMap<RustObject, Vec<(ObjectMetadata, ObjectUsage)>>,
    idents: IndexSet<RustIdent>,
}

impl Visit<'_> for CollectFieldlessEnums {
    fn visit_obj(&mut self, obj: &RustObject, meta: Option<&ObjectMetadata>, usage: ObjectUsage) {
        if let Some(meta) = meta {
            self.idents.insert(meta.ident.clone());
            if matches!(obj, RustObject::FieldlessEnum(_)) {
                self.objs.entry(obj.clone()).or_default().push((meta.clone(), usage));
            }
        }
        obj.visit(self, usage);
    }
}

struct ReplaceWithShared<'a> {
    canonical: &'a IndexMap<RustObject, RustIdent>,
    aliases: Vec<TypeAlias>,
    /// Copies in this component, used to keep the name the per-component pass gave them
    replaced: IndexMap<RustObject, Vec<(ObjectMetadata, ObjectUsage)>>,
}

impl ReplaceWithShared<'_> {
    /// Alias the name `deduplicate_types` used to infer for copies within one component
    fn alias_component_level_names(&mut self) {
        for (obj, meta) in &self.replaced {
            if meta.len() < 2 {
                continue;
            }
            let Some(info) = infer_dedupped_object_for(meta, obj) else {
                continue;
            };
            if meta.iter().any(|(m, _)| m.ident == info.ident) {
                continue;
            }
            self.aliases.push(TypeAlias {
                ident: info.ident,
                target: RustType::path(PathToType::Shared(self.canonical[obj].clone()), false),
                doc: None,
                kind: info.usage.kind,
            });
        }
    }
}

impl VisitMut for ReplaceWithShared<'_> {
    fn visit_typ_mut(&mut self, typ: &mut RustType, usage: ObjectUsage) {
        if let Some((obj, meta)) = typ.as_object_mut()
            && let Some(ident) = self.canonical.get(obj)
        {
            self.replaced.entry(obj.clone()).or_default().push((meta.clone(), usage));
            // Alias even when the name is unchanged, so the old module path keeps resolving
            let target = RustType::path(PathToType::Shared(ident.clone()), false);
            self.aliases.push(TypeAlias {
                ident: meta.ident.clone(),
                target: target.clone(),
                doc: meta.doc.clone(),
                kind: usage.kind,
            });
            *typ = target;
            return;
        }
        typ.visit_mut(self, usage);
    }
}

/// Pick a name for a group of identical enums.
///
/// Prefer the field name all copies share (`setup_future_usage` -> `SetupFutureUsage`). Fall
/// back to the shortest existing name, which is already unique to this group.
fn candidate_names(meta: &[(ObjectMetadata, ObjectUsage)]) -> Vec<RustIdent> {
    let mut names = vec![];
    let first_field = meta[0].0.field_name.as_ref();
    if let Some(field) = first_field
        && meta.iter().all(|(m, _)| m.field_name.as_ref() == Some(field))
    {
        names.push(RustIdent::create(field));
    }
    let mut own: Vec<&RustIdent> = meta.iter().map(|(m, _)| &m.ident).collect();
    own.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
    names.extend(own.into_iter().cloned());
    names
}

fn merged_usage(meta: &[(ObjectMetadata, ObjectUsage)]) -> ObjectUsage {
    let used_as_request_param = meta.iter().any(|(_, u)| u.used_as_request_param);
    let kind = if meta.iter().all(|(_, u)| u.kind == ObjectKind::RequestParam) {
        ObjectKind::RequestParam
    } else {
        ObjectKind::Type
    };
    ObjectUsage { kind, used_as_request_param }
}

fn common_doc(meta: &[(ObjectMetadata, ObjectUsage)]) -> Option<String> {
    let doc = meta[0].0.doc.as_ref()?;
    meta.iter().all(|(m, _)| m.doc.as_ref() == Some(doc)).then(|| doc.clone())
}

#[tracing::instrument(level = "debug", skip_all)]
pub fn deduplicate_across_components(components: &mut Components) {
    let mut collector = CollectFieldlessEnums::default();
    for comp in components.components.values() {
        collector.idents.insert(comp.ident().clone());
        collector.idents.insert(comp.id_type_ident());
        collector.idents.insert(comp.object_name_ident());
        comp.visit(&mut collector);
    }
    collector.idents.extend(components.extra_types.keys().cloned());

    let mut taken_mods: IndexSet<String> =
        components.components.values().map(|c| c.mod_path()).collect();
    // Written by `CodeGen::write_api_version_file`
    taken_mods.insert("version".into());
    taken_mods.extend(components.extra_types.values().map(|t| t.mod_path.clone()));

    // Names chosen for other groups, plus every name that exists today outside this group
    let mut chosen: IndexSet<RustIdent> = IndexSet::new();
    let mut canonical = IndexMap::new();
    let mut copies_removed = 0;
    for (obj, meta) in &collector.objs {
        if meta.len() < 2 {
            continue;
        }
        let own: IndexSet<&RustIdent> = meta.iter().map(|(m, _)| &m.ident).collect();
        let name = candidate_names(meta).into_iter().find(|name| {
            if PRELUDE_NAMES.contains(&name.as_ref()) {
                return false;
            }
            let clashes_elsewhere = collector.idents.contains(name) && !own.contains(name);
            !clashes_elsewhere && !chosen.contains(name) && !taken_mods.contains(&mod_path(name))
        });
        let Some(name) = name else {
            continue;
        };
        chosen.insert(name.clone());
        taken_mods.insert(mod_path(&name));
        copies_removed += meta.len() - 1;
        components.extra_types.insert(
            name.clone(),
            TypeSpec {
                obj: obj.clone(),
                doc: common_doc(meta),
                ident: name.clone(),
                mod_path: mod_path(&name),
                usage: merged_usage(meta),
            },
        );
        canonical.insert(obj.clone(), name);
    }

    for comp in components.components.values_mut() {
        let mut replacer =
            ReplaceWithShared { canonical: &canonical, aliases: vec![], replaced: IndexMap::new() };
        comp.visit_mut(&mut replacer);
        replacer.alias_component_level_names();
        comp.type_aliases = replacer.aliases;
    }
    info!(
        "Hoisted {} fieldless enums into stripe_shared, removing {copies_removed} copies",
        canonical.len()
    );
}

fn mod_path(ident: &RustIdent) -> String {
    let path = ident.to_snake_case();
    if KEYWORDS.contains(&path.as_str()) { format!("{path}_") } else { path }
}

/// Generated code names these unqualified, so a type with one of these names would shadow them.
pub(crate) const PRELUDE_NAMES: &[&str] =
    &["Result", "Option", "Some", "None", "Ok", "Err", "String", "Vec", "Box", "Default"];

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "box", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
    "try", "type", "unsafe", "use", "where", "while", "yield",
];
