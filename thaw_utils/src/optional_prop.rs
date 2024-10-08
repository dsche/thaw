use leptos::prelude::*;
use std::ops::{Deref, DerefMut};

use crate::BoxOneCallback;

pub struct OptionalProp<T>(Option<T>);

impl<T> Default for OptionalProp<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Clone> Clone for OptionalProp<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> Copy for OptionalProp<T> {}

impl<T> OptionalProp<T> {
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        self.0.map(f)
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> Deref for OptionalProp<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptionalProp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for OptionalProp<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl From<&str> for OptionalProp<String> {
    fn from(value: &str) -> Self {
        Self(Some(value.to_string()))
    }
}

/// TODO remove signal
impl From<&str> for OptionalProp<MaybeSignal<String>> {
    fn from(value: &str) -> Self {
        Self(Some(MaybeSignal::from(value.to_string())))
    }
}

impl From<String> for OptionalProp<MaybeSignal<String>> {
    fn from(value: String) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T: Send + Sync> From<ReadSignal<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: ReadSignal<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T: Send + Sync> From<RwSignal<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: RwSignal<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T: Send + Sync> From<Memo<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: Memo<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T, S> From<Signal<T, S>> for OptionalProp<MaybeSignal<T, S>>
where
    S: Storage<T>,
{
    fn from(value: Signal<T, S>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<F, A, Return> From<F> for OptionalProp<BoxOneCallback<A, Return>>
where
    F: Fn(A) -> Return + Send + Sync + 'static,
{
    fn from(value: F) -> Self {
        Self(Some(BoxOneCallback::new(value)))
    }
}

impl<T> From<Option<T>> for OptionalProp<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::OptionalProp;
    use leptos::prelude::MaybeSignal;

    #[test]
    fn from() {
        let _prop: OptionalProp<MaybeSignal<String>> = "prop".into();
        let _prop: OptionalProp<MaybeSignal<String>> = "prop".to_string().into();
        let _prop: OptionalProp<String> = "prop".into();
    }
}
