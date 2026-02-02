use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastVariant {
    fn css_class(&self) -> &'static str {
        match self {
            ToastVariant::Success => "toast toast-success",
            ToastVariant::Error => "toast toast-error",
            ToastVariant::Warning => "toast toast-warning",
            ToastVariant::Info => "toast toast-info",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ToastVariant::Success => "\u{2713}",
            ToastVariant::Error => "\u{2717}",
            ToastVariant::Warning => "\u{26a0}",
            ToastVariant::Info => "\u{2139}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u32,
    pub message: String,
    pub variant: ToastVariant,
}

#[derive(Clone, Copy)]
pub struct ToastDispatch {
    set_toasts: WriteSignal<Vec<Toast>>,
    next_id: RwSignal<u32>,
}

impl ToastDispatch {
    fn push(&self, message: String, variant: ToastVariant) {
        let id = self.next_id.get_untracked() + 1;
        self.next_id.set_untracked(id);

        let toast = Toast { id, message, variant };
        self.set_toasts.update(|list| list.push(toast));

        let set_toasts = self.set_toasts;
        set_timeout(
            move || {
                set_toasts.update(|list| list.retain(|t| t.id != id));
            },
            std::time::Duration::from_millis(4000),
        );
    }

    pub fn success(&self, msg: &str) {
        self.push(msg.to_string(), ToastVariant::Success);
    }

    pub fn error(&self, msg: &str) {
        self.push(msg.to_string(), ToastVariant::Error);
    }

    #[allow(dead_code)]
    pub fn warning(&self, msg: &str) {
        self.push(msg.to_string(), ToastVariant::Warning);
    }

    #[allow(dead_code)]
    pub fn info(&self, msg: &str) {
        self.push(msg.to_string(), ToastVariant::Info);
    }
}

pub fn provide_toast_context() {
    let (toasts, set_toasts) = create_signal(Vec::<Toast>::new());
    let next_id = create_rw_signal(0u32);

    let dispatch = ToastDispatch {
        set_toasts,
        next_id,
    };

    provide_context(toasts);
    provide_context(dispatch);
}

pub fn use_toast() -> ToastDispatch {
    expect_context::<ToastDispatch>()
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    let toasts = expect_context::<ReadSignal<Vec<Toast>>>();

    view! {
        <div class="toast-container">
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    let css = toast.variant.css_class().to_string();
                    let icon = toast.variant.icon().to_string();
                    view! {
                        <div class=css>
                            <span class="toast-icon">{icon}</span>
                            <span class="toast-message">{toast.message}</span>
                        </div>
                    }
                }
            />
        </div>
    }
}
