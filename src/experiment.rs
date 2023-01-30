use crate::ansi;
use crate::bars::PlotHighlight;
use crate::duration::Duration;
use crate::experiment_name::ExperimentName;
use crate::math::numbers::Numbers;
use crate::mem_usage::MemUsage;

pub struct Experiment {
    pub name: ExperimentName,
    pub warmup: String,
    pub run: String,
    pub durations: Numbers<Duration>,
    pub mem_usages: Numbers<MemUsage>,
}

impl Experiment {
    pub fn plot_highlights(&self) -> PlotHighlight {
        PlotHighlight {
            non_zero: format!("{}", self.name.color().to_owned()),
            zero: format!("{}", ansi::WHITE_BG),
            reset: ansi::RESET.to_owned(),
        }
    }

    pub fn plot_halves_highlights(&self) -> PlotHighlight {
        PlotHighlight {
            non_zero: format!("{}", self.name.color().to_owned()),
            zero: "".to_owned(),
            reset: ansi::RESET.to_owned(),
        }
    }

    pub fn runs(&self) -> usize {
        self.durations.len()
    }
}