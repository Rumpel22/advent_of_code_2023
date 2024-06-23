use nom::{
    bytes::complete::{tag, take},
    character::complete::alpha1,
    multi::separated_list0,
    IResult,
};
use std::{any::Any, cell::Cell, collections::HashMap};

#[derive(Clone, Copy, PartialEq)]
pub enum Signal {
    High,
    Low,
}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub trait Module: AsAny {
    fn recv_signal(&mut self, signal: &Signal, sender: &'static str)
        -> Vec<(&'static str, Signal)>;
    fn outputs(&self) -> &[&'static str];
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct FlipFlop<'a> {
    outputs: Vec<&'a str>,
    is_off: Cell<bool>,
}
impl Module for FlipFlop<'static> {
    fn recv_signal(&mut self, signal: &Signal, _: &'static str) -> Vec<(&'static str, Signal)> {
        match signal {
            Signal::High => vec![],
            Signal::Low => {
                self.is_off.set(!self.is_off.get());
                let new_signal = if self.is_off.get() {
                    Signal::Low
                } else {
                    Signal::High
                };
                self.outputs
                    .iter()
                    .map(|output| (*output, new_signal))
                    .collect::<Vec<_>>()
            }
        }
    }

    fn outputs(&self) -> &[&'static str] {
        &self.outputs
    }
}

struct Conjunction<'a> {
    outputs: Vec<&'a str>,
    inputs: HashMap<&'a str, Signal>,
}
impl Module for Conjunction<'static> {
    fn recv_signal(
        &mut self,
        signal: &Signal,
        sender: &'static str,
    ) -> Vec<(&'static str, Signal)> {
        *self.inputs.get_mut(sender).unwrap() = *signal;
        let new_signal = match self
            .inputs
            .iter()
            .all(|(_, signal)| *signal == Signal::High)
        {
            true => Signal::Low,
            false => Signal::High,
        };

        self.outputs
            .iter()
            .map(|output| (*output, new_signal))
            .collect()
    }

    fn outputs(&self) -> &[&'static str] {
        &self.outputs
    }
}
impl<'a> Conjunction<'a> {
    fn set_inputs(&mut self, inputs: &[&'a str]) {
        self.inputs = HashMap::from_iter(inputs.iter().map(|name| (*name, Signal::Low)));
    }
}

struct Broadcaster<'a> {
    outputs: Vec<&'a str>,
}
impl Module for Broadcaster<'static> {
    fn recv_signal(&mut self, signal: &Signal, _: &str) -> Vec<(&'static str, Signal)> {
        match signal {
            Signal::High => unreachable!(),
            Signal::Low => self
                .outputs
                .iter()
                .map(|output| (*output, *signal))
                .collect::<Vec<_>>(),
        }
    }

    fn outputs(&self) -> &[&'static str] {
        &self.outputs
    }
}

fn parse_module<'a>(input: &'static str) -> IResult<&str, (&'a str, Box<dyn Module>)> {
    let (input, module_type) = take(1u8)(input)?;
    let (input, mut name) = alpha1(input)?;
    let (input, _) = take(4u8)(input)?;
    let (input, outputs) = separated_list0(tag(", "), alpha1)(input)?;

    let module: Box<dyn Module> = match module_type {
        "%" => Box::new(FlipFlop {
            outputs,
            is_off: Cell::new(true),
        }),
        "&" => Box::new(Conjunction {
            outputs,
            inputs: HashMap::<_, _>::new(),
        }),
        "b" => {
            name = "broadcaster";
            Box::new(Broadcaster { outputs })
        }
        _ => unreachable!(),
    };

    Ok((input, (name, module)))
}

pub fn get_modules(input: &'static str) -> HashMap<&'static str, Box<dyn Module>> {
    let mut modules = input
        .lines()
        .map(|line| parse_module(line).unwrap().1)
        .collect::<HashMap<_, _>>();

    let conjunctions = modules
        .iter()
        .filter_map(|(name, module)| {
            let module_any = module.as_ref().as_any();
            if module_any.is::<Conjunction>() {
                Some(*name)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for conjunction in conjunctions {
        let inputs = modules
            .iter()
            .filter_map(|(name, module)| {
                if module.outputs().contains(&conjunction) {
                    Some(*name)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let module = modules.get_mut(conjunction).unwrap();
        let module_any = module.as_mut().as_mut_any();
        match module_any.downcast_mut::<Conjunction>() {
            Some(conjunction) => conjunction.set_inputs(&inputs),
            None => unreachable!(),
        }
    }

    modules
}
