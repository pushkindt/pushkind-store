use std::fmt;

#[derive(Clone, Default)]
pub enum AlertType {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl fmt::Display for AlertType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            AlertType::Primary => "alert-primary",
            AlertType::Secondary => "alert-secondary",
            AlertType::Success => "alert-success",
            AlertType::Danger => "alert-danger",
            AlertType::Warning => "alert-warning",
            AlertType::Info => "alert-info",
            AlertType::Light => "alert-light",
            AlertType::Dark => "alert-dark",
        };
        write!(f, "{}", repr)
    }
}

#[derive(Clone, Default)]
pub struct AlertMessage {
    pub message: String,
    pub alert_type: AlertType,
    pub visible: bool,
}
