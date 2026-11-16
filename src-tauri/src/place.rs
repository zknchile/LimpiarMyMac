//! Decide si, al redibujar, la grilla se queda donde estaba.

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub screen: String,
    pub active: String,
    pub list: Option<f64>,
    pub sidebar: Option<f64>,
    pub focus_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeptPlace {
    pub sidebar: Option<f64>,
    pub list: Option<f64>,
    pub focus_id: String,
}

pub fn place_to_keep(before: Option<&Place>, after_screen: &str, after_active: &str, confirm: bool) -> KeptPlace {
    let Some(before) = before else {
        return empty();
    };
    if before.screen != "results" || after_screen != "results" {
        return empty();
    }
    let same_list = before.active == after_active;
    KeptPlace {
        sidebar: before.sidebar,
        list: if same_list { before.list } else { None },
        focus_id: if same_list && !confirm {
            before.focus_id.clone()
        } else {
            String::new()
        },
    }
}

fn empty() -> KeptPlace {
    KeptPlace {
        sidebar: None,
        list: None,
        focus_id: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn before() -> Place {
        Place {
            screen: "results".into(),
            active: "caches".into(),
            list: Some(480.0),
            sidebar: Some(120.0),
            focus_id: "caches-4".into(),
        }
    }

    // TEST-place-001
    #[test]
    fn same_category_keeps_list_and_focus() {
        let kept = place_to_keep(Some(&before()), "results", "caches", false);
        assert_eq!(kept.list, Some(480.0));
        assert_eq!(kept.sidebar, Some(120.0));
        assert_eq!(kept.focus_id, "caches-4");
    }

    // TEST-place-002
    #[test]
    fn another_category_does_not_inherit_list_offset() {
        let kept = place_to_keep(Some(&before()), "results", "logs", false);
        assert_eq!(kept.list, None);
        assert_eq!(kept.sidebar, Some(120.0));
        assert_eq!(kept.focus_id, "");
    }

    // TEST-place-003
    #[test]
    fn leaving_results_or_opening_confirm_does_not_reuse_place() {
        let gone = place_to_keep(Some(&before()), "cleaning", "caches", false);
        assert_eq!(gone.list, None);
        assert_eq!(gone.sidebar, None);
        assert_eq!(gone.focus_id, "");
        let sheet = place_to_keep(Some(&before()), "results", "caches", true);
        assert_eq!(sheet.list, Some(480.0));
        assert_eq!(sheet.focus_id, "");
    }
}
