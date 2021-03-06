use crate::processor::{ProcessValue, ProcessingState, Processor};
use crate::types::{Array, Empty, Error, Meta, Object, ValueAction};

pub struct SchemaProcessor;

impl Processor for SchemaProcessor {
    fn process_string(
        &mut self,
        value: &mut String,
        meta: &mut Meta,
        state: &ProcessingState<'_>,
    ) -> ValueAction {
        verify_value_nonempty(value, meta, &state)
            .and_then(|| verify_value_pattern(value, meta, &state))
    }

    fn process_array<T>(
        &mut self,
        value: &mut Array<T>,
        meta: &mut Meta,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        T: ProcessValue,
    {
        value.process_child_values(self, state);
        verify_value_nonempty(value, meta, state)
    }

    fn process_object<T>(
        &mut self,
        value: &mut Object<T>,
        meta: &mut Meta,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        T: ProcessValue,
    {
        value.process_child_values(self, state);
        verify_value_nonempty(value, meta, state)
    }
}

fn verify_value_nonempty<T>(
    value: &mut T,
    meta: &mut Meta,
    state: &ProcessingState<'_>,
) -> ValueAction
where
    T: Empty,
{
    if state.attrs().nonempty && value.is_empty() {
        meta.add_error(Error::expected("non-empty value"));
        ValueAction::DeleteHard
    } else {
        ValueAction::Keep
    }
}

fn verify_value_pattern(
    value: &mut String,
    meta: &mut Meta,
    state: &ProcessingState<'_>,
) -> ValueAction {
    if let Some(ref regex) = state.attrs().match_regex {
        if !regex.is_match(value) {
            meta.add_error(Error::invalid("invalid characters in string"));
            return ValueAction::DeleteSoft;
        }
    }

    ValueAction::Keep
}

#[cfg(test)]
mod tests {
    use super::SchemaProcessor;
    use crate::processor::{process_value, ProcessingState};
    use crate::types::{Annotated, Array, Error, Object, Value};

    fn assert_nonempty_base<T>()
    where
        T: Default + PartialEq + crate::processor::ProcessValue,
    {
        #[derive(Clone, Debug, Default, PartialEq, Empty, FromValue, ToValue, ProcessValue)]
        struct Foo<T> {
            #[metastructure(required = "true", nonempty = "true")]
            bar: Annotated<T>,
            bar2: Annotated<T>,
        }

        let mut wrapper = Annotated::new(Foo {
            bar: Annotated::new(T::default()),
            bar2: Annotated::new(T::default()),
        });
        process_value(&mut wrapper, &mut SchemaProcessor, ProcessingState::root());

        assert_eq_dbg!(
            wrapper,
            Annotated::new(Foo {
                bar: Annotated::from_error(Error::expected("non-empty value"), None),
                bar2: Annotated::new(T::default())
            })
        );
    }

    #[test]
    fn test_nonempty_string() {
        assert_nonempty_base::<String>();
    }

    #[test]
    fn test_nonempty_array() {
        assert_nonempty_base::<Array<u64>>();
    }

    #[test]
    fn test_nonempty_object() {
        assert_nonempty_base::<Object<u64>>();
    }

    #[test]
    fn test_release_newlines() {
        use crate::protocol::Event;

        let mut event = Annotated::new(Event {
            release: Annotated::new("a\nb".to_string()),
            ..Default::default()
        });

        process_value(&mut event, &mut SchemaProcessor, ProcessingState::root());

        assert_eq_dbg!(
            event,
            Annotated::new(Event {
                release: Annotated::from_error(
                    Error::invalid("invalid characters in string"),
                    Some(Value::String("a\nb".into())),
                ),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_invalid_email() {
        use crate::protocol::User;

        let mut user = Annotated::new(User {
            email: Annotated::new("bananabread".to_owned()),
            ..Default::default()
        });

        process_value(&mut user, &mut SchemaProcessor, ProcessingState::root());

        assert_eq_dbg!(
            user,
            Annotated::new(User {
                email: Annotated::from_error(
                    Error::invalid("invalid characters in string"),
                    Some(Value::String("bananabread".to_string()))
                ),
                ..Default::default()
            })
        );
    }
}
