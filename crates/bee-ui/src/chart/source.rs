use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::data::TimeSeriesDataset;

pub trait PlotDataSource: Send + Sync {
    fn snapshot(&self) -> TimeSeriesDataset;
}

#[derive(Clone, Debug)]
pub struct SharedPlotSource {
    inner: Arc<RwLock<TimeSeriesDataset>>,
}

impl SharedPlotSource {
    pub fn new(dataset: TimeSeriesDataset) -> Self {
        Self {
            inner: Arc::new(RwLock::new(dataset)),
        }
    }

    pub fn from_arc(inner: Arc<RwLock<TimeSeriesDataset>>) -> Self {
        Self { inner }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, TimeSeriesDataset> {
        self.inner
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, TimeSeriesDataset> {
        self.inner
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn snapshot(&self) -> TimeSeriesDataset {
        self.read().clone()
    }

    pub fn inner(&self) -> &Arc<RwLock<TimeSeriesDataset>> {
        &self.inner
    }

    pub fn into_inner(self) -> Arc<RwLock<TimeSeriesDataset>> {
        self.inner
    }
}

impl Default for SharedPlotSource {
    fn default() -> Self {
        Self::new(TimeSeriesDataset::default())
    }
}

impl PlotDataSource for SharedPlotSource {
    fn snapshot(&self) -> TimeSeriesDataset {
        SharedPlotSource::snapshot(self)
    }
}

pub type SharedTimeSeriesDataset = SharedPlotSource;
