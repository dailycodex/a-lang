#![allow(unused)]
use super::Span;
use std::any::Any;
use std::fmt::Debug;

pub trait Token: Any + Debug {
    fn new(value: String, span: Span) -> Self
    where
        Self: Sized;
    fn value(&self) -> String;
    fn span(&self) -> Span;
    fn as_any(&self) -> &dyn Any;
}
