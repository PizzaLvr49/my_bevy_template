use bevy::prelude::*;
use std::sync::Arc;

pub trait PanicHandleFn<Res>:
    Fn(&std::panic::PanicHookInfo) -> Res + Send + Sync + 'static
{
}

impl<Res, T: Fn(&std::panic::PanicHookInfo) -> Res + Send + Sync + 'static> PanicHandleFn<Res>
    for T
{
}

#[derive(Default)]
pub struct PanicHandlerBuilder {
    title: Option<Arc<dyn PanicHandleFn<String>>>,
    body: Option<Arc<dyn PanicHandleFn<String>>>,
    hook: Option<Arc<dyn PanicHandleFn<()>>>,
}

impl PanicHandlerBuilder {
    #[must_use]
    pub fn build(self) -> PanicHandler {
        PanicHandler {
            title: self.title.unwrap_or_else(|| {
                Arc::new(|_: &std::panic::PanicHookInfo| "Fatal Error".to_owned())
            }),
            body: self.body.unwrap_or_else(|| {
                Arc::new(|info| {
                    format!(
                        "Unhandled panic at {}:\n{}",
                        info.location()
                            .map_or("Unknown Location".to_owned(), ToString::to_string),
                        info.payload()
                            .downcast_ref::<String>()
                            .map(|s| s.as_str())
                            .or_else(|| info.payload().downcast_ref::<&str>().copied())
                            .unwrap_or("No panic message available")
                    )
                })
            }),
            hook: self.hook.unwrap_or_else(|| Arc::new(|_| {})),
        }
    }

    #[must_use]
    pub fn take_call_from_existing(mut self) -> Self {
        self.hook = Some(Arc::new(std::panic::take_hook()));
        self
    }

    #[must_use]
    pub fn set_call_func(mut self, call_func: impl PanicHandleFn<()>) -> Self {
        self.hook = Some(Arc::new(call_func));
        self
    }

    #[must_use]
    pub fn set_title_func(mut self, title_func: impl PanicHandleFn<String>) -> Self {
        self.title = Some(Arc::new(title_func));
        self
    }

    #[must_use]
    pub fn set_body_func(mut self, body_func: impl PanicHandleFn<String>) -> Self {
        self.body = Some(Arc::new(body_func));
        self
    }
}

#[derive(Clone)]
pub struct PanicHandler {
    pub title: Arc<dyn PanicHandleFn<String>>,
    pub body: Arc<dyn PanicHandleFn<String>>,
    pub hook: Arc<dyn PanicHandleFn<()>>,
}

impl PanicHandler {
    #[must_use]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> PanicHandlerBuilder {
        PanicHandlerBuilder::default()
    }

    #[must_use]
    pub fn new_take_old() -> PanicHandlerBuilder {
        PanicHandlerBuilder::default().take_call_from_existing()
    }
}

impl Default for PanicHandler {
    fn default() -> Self {
        PanicHandler::new().build()
    }
}

impl Plugin for PanicHandler {
    fn build(&self, _app: &mut App) {
        let handler = self.clone();
        std::panic::set_hook(Box::new(move |info| {
            let title_string = (handler.title)(info);
            let body_string = (handler.body)(info);

            error!("{}\n{}", title_string, body_string);

            #[cfg(any(target_os = "windows", target_os = "macos", target_family = "unix"))]
            {
                rfd::MessageDialog::new()
                    .set_title(&title_string)
                    .set_description(&body_string)
                    .set_level(rfd::MessageLevel::Error)
                    .show();
            }

            (handler.hook)(info);
        }));
    }
}
