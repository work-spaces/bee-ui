use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

#[path = "../../assets/fonts.rs"]
mod sources;

/// The supported embedded runnerd font faces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFace {
    PropoRegular,
    PropoBold,
    PropoBoldItalic,
    PropoExtraLight,
    PropoExtraLightItalic,
    PropoItalic,
    PropoLight,
    PropoLightItalic,
    PropoMedium,
    PropoMediumItalic,
    PropoSemiBold,
    PropoSemiBoldItalic,
    PropoText,
    PropoTextItalic,
    PropoThin,
    PropoThinItalic,
    MonoRegular,
    MonoBold,
    MonoBoldItalic,
    MonoExtraBold,
    MonoExtraBoldItalic,
    MonoExtraLight,
    MonoExtraLightItalic,
    MonoItalic,
    MonoLight,
    MonoLightItalic,
    MonoMedium,
    MonoMediumItalic,
    MonoSemiBold,
    MonoSemiBoldItalic,
    MonoThin,
    MonoThinItalic,
}

impl FontFace {
    pub const ALL: [Self; 32] = [
        Self::PropoRegular,
        Self::PropoBold,
        Self::PropoBoldItalic,
        Self::PropoExtraLight,
        Self::PropoExtraLightItalic,
        Self::PropoItalic,
        Self::PropoLight,
        Self::PropoLightItalic,
        Self::PropoMedium,
        Self::PropoMediumItalic,
        Self::PropoSemiBold,
        Self::PropoSemiBoldItalic,
        Self::PropoText,
        Self::PropoTextItalic,
        Self::PropoThin,
        Self::PropoThinItalic,
        Self::MonoRegular,
        Self::MonoBold,
        Self::MonoBoldItalic,
        Self::MonoExtraBold,
        Self::MonoExtraBoldItalic,
        Self::MonoExtraLight,
        Self::MonoExtraLightItalic,
        Self::MonoItalic,
        Self::MonoLight,
        Self::MonoLightItalic,
        Self::MonoMedium,
        Self::MonoMediumItalic,
        Self::MonoSemiBold,
        Self::MonoSemiBoldItalic,
        Self::MonoThin,
        Self::MonoThinItalic,
    ];

    pub fn enum_key(self) -> &'static str {
        match self {
            Self::PropoRegular => "PropoRegular",
            Self::PropoBold => "PropoBold",
            Self::PropoBoldItalic => "PropoBoldItalic",
            Self::PropoExtraLight => "PropoExtraLight",
            Self::PropoExtraLightItalic => "PropoExtraLightItalic",
            Self::PropoItalic => "PropoItalic",
            Self::PropoLight => "PropoLight",
            Self::PropoLightItalic => "PropoLightItalic",
            Self::PropoMedium => "PropoMedium",
            Self::PropoMediumItalic => "PropoMediumItalic",
            Self::PropoSemiBold => "PropoSemiBold",
            Self::PropoSemiBoldItalic => "PropoSemiBoldItalic",
            Self::PropoText => "PropoText",
            Self::PropoTextItalic => "PropoTextItalic",
            Self::PropoThin => "PropoThin",
            Self::PropoThinItalic => "PropoThinItalic",
            Self::MonoRegular => "MonoRegular",
            Self::MonoBold => "MonoBold",
            Self::MonoBoldItalic => "MonoBoldItalic",
            Self::MonoExtraBold => "MonoExtraBold",
            Self::MonoExtraBoldItalic => "MonoExtraBoldItalic",
            Self::MonoExtraLight => "MonoExtraLight",
            Self::MonoExtraLightItalic => "MonoExtraLightItalic",
            Self::MonoItalic => "MonoItalic",
            Self::MonoLight => "MonoLight",
            Self::MonoLightItalic => "MonoLightItalic",
            Self::MonoMedium => "MonoMedium",
            Self::MonoMediumItalic => "MonoMediumItalic",
            Self::MonoSemiBold => "MonoSemiBold",
            Self::MonoSemiBoldItalic => "MonoSemiBoldItalic",
            Self::MonoThin => "MonoThin",
            Self::MonoThinItalic => "MonoThinItalic",
        }
    }

    pub fn manifest_key(self) -> &'static str {
        entry_for_face(self).manifest_key
    }
}

#[derive(Clone, Copy)]
struct FaceEntry {
    manifest_key: &'static str,
    source_font: sources::Font,
}

static FACE_CACHE: OnceLock<Mutex<HashMap<FontFace, Arc<egui::FontData>>>> = OnceLock::new();
const STAGED_FONT_DEFINITIONS_ID: &str = "runnerd_bs.theme.fonts.staged_definitions";

/// Returns a named family selector for a specific runnerd face.
///
/// Consumers can apply this to `egui::RichText::family(...)` after calling
/// [`ensure_face_registered`] for the same face.
pub fn font_family_for(face: FontFace) -> egui::FontFamily {
    egui::FontFamily::Name(face.manifest_key().into())
}

/// Ensures a specific face is installed in the current `egui::Context`.
///
/// Returns `true` when the face is active in the current pass and can be used immediately with
/// [`font_family_for`]. Returns `false` when the face was newly queued (or already queued) and
/// will be available on a subsequent pass.
pub fn ensure_face_registered(ctx: &egui::Context, face: FontFace) -> bool {
    if face_is_active(ctx, face) {
        return true;
    }

    let mut definitions = current_or_staged_font_definitions(ctx);
    if definitions_has_face(&definitions, face) {
        return false;
    }

    let font_name = face.manifest_key().to_owned();
    let face_family = font_family_for(face);

    definitions
        .font_data
        .entry(font_name.clone())
        .or_insert_with(|| load_face(face));

    let family = definitions.families.entry(face_family).or_default();
    family.retain(|name| name != &font_name);
    family.insert(0, font_name);

    apply_staged_font_definitions(ctx, definitions);
    ctx.request_repaint();

    false
}

/// Lazily loads an embedded runnerd face from project assets.
///
/// On first use for a face, this function builds `egui::FontData` and stores it in a global
/// cache. Future calls are cache hits.
pub fn load_face(face: FontFace) -> Arc<egui::FontData> {
    let mut cache = font_face_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if let Some(cached) = cache.get(&face) {
        return Arc::clone(cached);
    }

    let loaded = Arc::new(font_data_for_face(face));
    cache.insert(face, Arc::clone(&loaded));
    loaded
}

pub(super) fn current_or_staged_font_definitions(ctx: &egui::Context) -> egui::FontDefinitions {
    let staged_id = egui::Id::new(STAGED_FONT_DEFINITIONS_ID);
    if let Some(staged) = ctx.data_mut(|data| data.get_temp::<egui::FontDefinitions>(staged_id)) {
        return staged;
    }

    if let Ok(definitions) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ctx.fonts(|fonts| fonts.definitions().clone())
    })) {
        return definitions;
    }

    egui::FontDefinitions::default()
}

pub(super) fn apply_staged_font_definitions(
    ctx: &egui::Context,
    definitions: egui::FontDefinitions,
) {
    let staged_id = egui::Id::new(STAGED_FONT_DEFINITIONS_ID);
    ctx.data_mut(|data| data.insert_temp(staged_id, definitions.clone()));
    ctx.set_fonts(definitions);
}

fn font_face_cache() -> &'static Mutex<HashMap<FontFace, Arc<egui::FontData>>> {
    FACE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn face_is_active(ctx: &egui::Context, face: FontFace) -> bool {
    let Ok(active) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let family = font_family_for(face);
        ctx.fonts(|fonts| {
            fonts
                .definitions()
                .families
                .get(&family)
                .is_some_and(|font_names| font_names.iter().any(|name| name == face.manifest_key()))
        })
    })) else {
        return false;
    };

    active
}

fn definitions_has_face(definitions: &egui::FontDefinitions, face: FontFace) -> bool {
    let family = font_family_for(face);
    definitions
        .families
        .get(&family)
        .is_some_and(|font_names| font_names.iter().any(|name| name == face.manifest_key()))
}

fn font_data_for_face(face: FontFace) -> egui::FontData {
    let entry = entry_for_face(face);
    egui::FontData::from_static(sources::include_font(entry.source_font))
}

fn entry_for_face(face: FontFace) -> FaceEntry {
    match face {
        FontFace::PropoRegular => FaceEntry {
            manifest_key: "PropoRegular",
            source_font: sources::Font::Propo(sources::Propo::Regular),
        },
        FontFace::PropoBold => FaceEntry {
            manifest_key: "PropoBold",
            source_font: sources::Font::Propo(sources::Propo::Bold),
        },
        FontFace::PropoBoldItalic => FaceEntry {
            manifest_key: "PropoBoldItalic",
            source_font: sources::Font::Propo(sources::Propo::BoldItalic),
        },
        FontFace::PropoExtraLight => FaceEntry {
            manifest_key: "PropoExtraLight",
            source_font: sources::Font::Propo(sources::Propo::ExtraLight),
        },
        FontFace::PropoExtraLightItalic => FaceEntry {
            manifest_key: "PropoExtraLightItalic",
            source_font: sources::Font::Propo(sources::Propo::ExtraLightItalic),
        },
        FontFace::PropoItalic => FaceEntry {
            manifest_key: "PropoItalic",
            source_font: sources::Font::Propo(sources::Propo::Italic),
        },
        FontFace::PropoLight => FaceEntry {
            manifest_key: "PropoLight",
            source_font: sources::Font::Propo(sources::Propo::Light),
        },
        FontFace::PropoLightItalic => FaceEntry {
            manifest_key: "PropoLightItalic",
            source_font: sources::Font::Propo(sources::Propo::LightItalic),
        },
        FontFace::PropoMedium => FaceEntry {
            manifest_key: "PropoMedium",
            source_font: sources::Font::Propo(sources::Propo::Medium),
        },
        FontFace::PropoMediumItalic => FaceEntry {
            manifest_key: "PropoMediumItalic",
            source_font: sources::Font::Propo(sources::Propo::MediumItalic),
        },
        FontFace::PropoSemiBold => FaceEntry {
            manifest_key: "PropoSemiBold",
            source_font: sources::Font::Propo(sources::Propo::SemiBold),
        },
        FontFace::PropoSemiBoldItalic => FaceEntry {
            manifest_key: "PropoSemiBoldItalic",
            source_font: sources::Font::Propo(sources::Propo::SemiBoldItalic),
        },
        FontFace::PropoText => FaceEntry {
            manifest_key: "PropoText",
            source_font: sources::Font::Propo(sources::Propo::Text),
        },
        FontFace::PropoTextItalic => FaceEntry {
            manifest_key: "PropoTextItalic",
            source_font: sources::Font::Propo(sources::Propo::TextItalic),
        },
        FontFace::PropoThin => FaceEntry {
            manifest_key: "PropoThin",
            source_font: sources::Font::Propo(sources::Propo::Thin),
        },
        FontFace::PropoThinItalic => FaceEntry {
            manifest_key: "PropoThinItalic",
            source_font: sources::Font::Propo(sources::Propo::ThinItalic),
        },
        FontFace::MonoRegular => FaceEntry {
            manifest_key: "MonoRegular",
            source_font: sources::Font::Mono(sources::Mono::Regular),
        },
        FontFace::MonoBold => FaceEntry {
            manifest_key: "MonoBold",
            source_font: sources::Font::Mono(sources::Mono::Bold),
        },
        FontFace::MonoBoldItalic => FaceEntry {
            manifest_key: "MonoBoldItalic",
            source_font: sources::Font::Mono(sources::Mono::BoldItalic),
        },
        FontFace::MonoExtraBold => FaceEntry {
            manifest_key: "MonoExtraBold",
            source_font: sources::Font::Mono(sources::Mono::ExtraBold),
        },
        FontFace::MonoExtraBoldItalic => FaceEntry {
            manifest_key: "MonoExtraBoldItalic",
            source_font: sources::Font::Mono(sources::Mono::ExtraBoldItalic),
        },
        FontFace::MonoExtraLight => FaceEntry {
            manifest_key: "MonoExtraLight",
            source_font: sources::Font::Mono(sources::Mono::ExtraLight),
        },
        FontFace::MonoExtraLightItalic => FaceEntry {
            manifest_key: "MonoExtraLightItalic",
            source_font: sources::Font::Mono(sources::Mono::ExtraLightItalic),
        },
        FontFace::MonoItalic => FaceEntry {
            manifest_key: "MonoItalic",
            source_font: sources::Font::Mono(sources::Mono::Italic),
        },
        FontFace::MonoLight => FaceEntry {
            manifest_key: "MonoLight",
            source_font: sources::Font::Mono(sources::Mono::Light),
        },
        FontFace::MonoLightItalic => FaceEntry {
            manifest_key: "MonoLightItalic",
            source_font: sources::Font::Mono(sources::Mono::LightItalic),
        },
        FontFace::MonoMedium => FaceEntry {
            manifest_key: "MonoMedium",
            source_font: sources::Font::Mono(sources::Mono::Medium),
        },
        FontFace::MonoMediumItalic => FaceEntry {
            manifest_key: "MonoMediumItalic",
            source_font: sources::Font::Mono(sources::Mono::MediumItalic),
        },
        FontFace::MonoSemiBold => FaceEntry {
            manifest_key: "MonoSemiBold",
            source_font: sources::Font::Mono(sources::Mono::SemiBold),
        },
        FontFace::MonoSemiBoldItalic => FaceEntry {
            manifest_key: "MonoSemiBoldItalic",
            source_font: sources::Font::Mono(sources::Mono::SemiBoldItalic),
        },
        FontFace::MonoThin => FaceEntry {
            manifest_key: "MonoThin",
            source_font: sources::Font::Mono(sources::Mono::Thin),
        },
        FontFace::MonoThinItalic => FaceEntry {
            manifest_key: "MonoThinItalic",
            source_font: sources::Font::Mono(sources::Mono::ThinItalic),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_face_caches_by_enum_value() {
        let first = load_face(FontFace::PropoRegular);
        let second = load_face(FontFace::PropoRegular);

        assert!(Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn all_source_faces_are_covered_by_font_face() {
        let mut covered = std::collections::HashSet::new();

        for face in FontFace::ALL {
            covered.insert(entry_for_face(face).source_font);
        }

        assert_eq!(
            covered.len(),
            sources::Mono::ALL.len() + sources::Propo::ALL.len()
        );

        for mono in sources::Mono::ALL {
            assert!(covered.contains(&sources::Font::Mono(*mono)));
        }

        for propo in sources::Propo::ALL {
            assert!(covered.contains(&sources::Font::Propo(*propo)));
        }
    }
}
