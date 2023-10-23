use crate::{
    div, AnyElement, Bounds, Div, DivState, Element, ElementFocus, ElementId, ElementInteraction,
    FocusDisabled, FocusEnabled, FocusListeners, Focusable, IntoAnyElement, LayoutId, Pixels,
    SharedString, StatefulInteraction, StatefulInteractive, StatelessInteraction,
    StatelessInteractive, StyleRefinement, Styled, ViewContext,
};
use util::ResultExt;

pub struct Svg<
    V: 'static + Send + Sync,
    I: ElementInteraction<V> = StatelessInteraction<V>,
    F: ElementFocus<V> = FocusDisabled,
> {
    base: Div<V, I, F>,
    path: Option<SharedString>,
}

pub fn svg<V>() -> Svg<V, StatelessInteraction<V>, FocusDisabled>
where
    V: 'static + Send + Sync,
{
    Svg {
        base: div(),
        path: None,
    }
}

impl<V, I, F> Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl<V, F> Svg<V, StatelessInteraction<V>, F>
where
    V: 'static + Send + Sync,
    F: ElementFocus<V>,
{
    pub fn id(self, id: impl Into<ElementId>) -> Svg<V, StatefulInteraction<V>, F> {
        Svg {
            base: self.base.id(id),
            path: self.path,
        }
    }
}

impl<V, I, F> IntoAnyElement<V> for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    fn into_any(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V, I, F> Element for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    type ViewState = V;
    type ElementState = DivState;

    fn id(&self) -> Option<crate::ElementId> {
        self.base.id()
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        self.base.initialize(view_state, element_state, cx)
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> LayoutId {
        self.base.layout(view_state, element_state, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) where
        Self: Sized,
    {
        self.base.paint(bounds, view, element_state, cx);
        let color = self
            .base
            .compute_style(bounds, element_state, cx)
            .text
            .color;
        if let Some((path, color)) = self.path.as_ref().zip(color) {
            cx.paint_svg(bounds, path.clone(), color).log_err();
        }
    }
}

impl<V, I, F> Styled for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<V, I, F> StatelessInteractive for Svg<V, I, F>
where
    V: 'static + Send + Sync,
    I: ElementInteraction<V>,
    F: ElementFocus<V>,
{
    fn stateless_interactivity(&mut self) -> &mut StatelessInteraction<V> {
        self.base.stateless_interactivity()
    }
}

impl<V, F> StatefulInteractive for Svg<V, StatefulInteraction<V>, F>
where
    V: 'static + Send + Sync,
    F: ElementFocus<V>,
{
    fn stateful_interaction(&mut self) -> &mut StatefulInteraction<Self::ViewState> {
        self.base.stateful_interaction()
    }
}

impl<V, I> Focusable for Svg<V, I, FocusEnabled<V>>
where
    V: 'static + Send + Sync,
    I: ElementInteraction<V>,
{
    fn focus_listeners(&mut self) -> &mut FocusListeners<Self::ViewState> {
        self.base.focus_listeners()
    }

    fn set_focus_style(&mut self, style: StyleRefinement) {
        self.base.set_focus_style(style)
    }

    fn set_focus_in_style(&mut self, style: StyleRefinement) {
        self.base.set_focus_in_style(style)
    }

    fn set_in_focus_style(&mut self, style: StyleRefinement) {
        self.base.set_in_focus_style(style)
    }
}
