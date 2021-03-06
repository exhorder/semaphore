use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::processor::{process_value, ProcessValue, ProcessingState, Processor, ValueType};
use crate::types::{Annotated, Array, Meta, Object, Value, ValueAction};

impl ProcessValue for String {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::String)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_string(self, meta, state)
    }
}

impl ProcessValue for bool {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::Boolean)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_bool(self, meta, state)
    }
}

impl ProcessValue for u64 {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::Number)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_u64(self, meta, state)
    }
}

impl ProcessValue for i64 {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::Number)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_i64(self, meta, state)
    }
}

impl ProcessValue for f64 {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::Number)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_f64(self, meta, state)
    }
}

impl ProcessValue for DateTime<Utc> {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::DateTime)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_timestamp(self, meta, state)
    }
}

impl ProcessValue for Uuid {}

impl<T> ProcessValue for Array<T>
where
    T: ProcessValue,
{
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::Array)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_array(self, meta, state)
    }

    #[inline]
    fn process_child_values<P>(&mut self, processor: &mut P, state: &ProcessingState<'_>)
    where
        P: Processor,
    {
        for (index, element) in self.iter_mut().enumerate() {
            process_value(
                element,
                processor,
                &state.enter_index(index, state.inner_attrs(), ValueType::for_field(element)),
            );
        }
    }
}

impl<T> ProcessValue for Object<T>
where
    T: ProcessValue,
{
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        Some(ValueType::Object)
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        processor.process_object(self, meta, state)
    }

    #[inline]
    fn process_child_values<P>(&mut self, processor: &mut P, state: &ProcessingState<'_>)
    where
        P: Processor,
    {
        for (k, v) in self.iter_mut() {
            process_value(
                v,
                processor,
                &state.enter_borrowed(k, state.inner_attrs(), ValueType::for_field(v)),
            );
        }
    }
}

impl<T> ProcessValue for Box<T>
where
    T: ProcessValue,
{
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        (**self).value_type()
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        ProcessValue::process_value(self.as_mut(), meta, processor, state)
    }
}

impl ProcessValue for Value {
    #[inline]
    fn value_type(&self) -> Option<ValueType> {
        match self {
            Value::Null => None,
            Value::Bool(_) => Some(ValueType::Boolean),
            Value::I64(_) => Some(ValueType::Number),
            Value::U64(_) => Some(ValueType::Number),
            Value::F64(_) => Some(ValueType::Number),
            Value::String(_) => Some(ValueType::String),
            Value::Array(_) => Some(ValueType::Array),
            Value::Object(_) => Some(ValueType::Object),
        }
    }

    #[inline]
    fn process_value<P>(
        &mut self,
        meta: &mut Meta,
        processor: &mut P,
        state: &ProcessingState<'_>,
    ) -> ValueAction
    where
        P: Processor,
    {
        match self {
            Value::Null => Default::default(),
            Value::Bool(value) => ProcessValue::process_value(value, meta, processor, state),
            Value::I64(value) => ProcessValue::process_value(value, meta, processor, state),
            Value::U64(value) => ProcessValue::process_value(value, meta, processor, state),
            Value::F64(value) => ProcessValue::process_value(value, meta, processor, state),
            Value::String(value) => ProcessValue::process_value(value, meta, processor, state),
            Value::Array(value) => ProcessValue::process_value(value, meta, processor, state),
            Value::Object(value) => ProcessValue::process_value(value, meta, processor, state),
        }
    }
}

macro_rules! process_tuple {
    ($($name: ident),+) => {
        impl< $( $name: ProcessValue ),* > ProcessValue for ( $( Annotated<$name>, )* ) {
            #[inline]
            fn value_type(&self) -> Option<ValueType> {
                Some(ValueType::Array)
            }

            #[inline]
            #[allow(non_snake_case, unused_assignments)]
            fn process_child_values<P>(&mut self, processor: &mut P, state: &ProcessingState<'_>)
            where
                P: Processor,
            {
                let ($(ref mut $name,)*) = *self;
                let mut index = 0;

                $(
                    process_value($name, processor, &state.enter_index(index, state.inner_attrs(), ValueType::for_field($name)));
                    index += 1;
                )*
            }
        }
    };
}

process_tuple!(T1);
process_tuple!(T1, T2);
process_tuple!(T1, T2, T3);
process_tuple!(T1, T2, T3, T4);
process_tuple!(T1, T2, T3, T4, T5);
process_tuple!(T1, T2, T3, T4, T5, T6);
process_tuple!(T1, T2, T3, T4, T5, T6, T7);
process_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
process_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
process_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
process_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
process_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
