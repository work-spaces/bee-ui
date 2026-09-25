#![allow(dead_code)]

use crate::prelude::*;

use super::super::VARIANTS;

#[derive(Clone, Copy, Debug)]
pub(super) struct ClusterFixture {
    pub name: &'static str,
    pub region: &'static str,
    pub control_plane_version: &'static str,
    pub nodes: u16,
    pub status: &'static str,
    pub status_variant: Variant,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ServiceFixture {
    pub name: &'static str,
    pub cluster: &'static str,
    pub deploy_strategy: &'static str,
    pub desired_replicas: u16,
    pub ready_replicas: u16,
    pub p95_latency_ms: f32,
    pub error_rate_pct: f32,
    pub status: &'static str,
    pub status_variant: Variant,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct IncidentFixture {
    pub id: &'static str,
    pub cluster: &'static str,
    pub service: &'static str,
    pub severity: &'static str,
    pub summary: &'static str,
    pub state: &'static str,
    pub state_variant: Variant,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct MetricPoint {
    pub minute: u16,
    pub value: f64,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct MetricSeries {
    pub key: &'static str,
    pub label: &'static str,
    pub unit: &'static str,
    pub variant: Variant,
    pub points: &'static [MetricPoint],
}

pub(super) const CLUSTERS: [ClusterFixture; 3] = [
    ClusterFixture {
        name: "prod-a",
        region: "us-east-1",
        control_plane_version: "v1.34.2",
        nodes: 148,
        status: "Healthy",
        status_variant: Variant::Success,
    },
    ClusterFixture {
        name: "prod-b",
        region: "us-west-2",
        control_plane_version: "v1.34.2",
        nodes: 131,
        status: "Degraded",
        status_variant: Variant::Warning,
    },
    ClusterFixture {
        name: "staging",
        region: "us-central-1",
        control_plane_version: "v1.35.0-rc1",
        nodes: 42,
        status: "Canary",
        status_variant: Variant::Info,
    },
];

pub(super) const SERVICES: [ServiceFixture; 3] = [
    ServiceFixture {
        name: "scheduler",
        cluster: "prod-a",
        deploy_strategy: "rolling",
        desired_replicas: 12,
        ready_replicas: 12,
        p95_latency_ms: 42.0,
        error_rate_pct: 0.03,
        status: "Healthy",
        status_variant: Variant::Success,
    },
    ServiceFixture {
        name: "api-gateway",
        cluster: "prod-b",
        deploy_strategy: "canary",
        desired_replicas: 18,
        ready_replicas: 16,
        p95_latency_ms: 121.0,
        error_rate_pct: 0.70,
        status: "Degraded",
        status_variant: Variant::Warning,
    },
    ServiceFixture {
        name: "worker",
        cluster: "staging",
        deploy_strategy: "rollback",
        desired_replicas: 40,
        ready_replicas: 37,
        p95_latency_ms: 67.0,
        error_rate_pct: 0.10,
        status: "Recovering",
        status_variant: Variant::Info,
    },
];

pub(super) const INCIDENTS: [IncidentFixture; 3] = [
    IncidentFixture {
        id: "INC-1042",
        cluster: "prod-b",
        service: "api-gateway",
        severity: "warning",
        summary: "Elevated p95 latency breached SLO for 12m",
        state: "Investigating",
        state_variant: Variant::Warning,
    },
    IncidentFixture {
        id: "INC-1043",
        cluster: "prod-a",
        service: "scheduler",
        severity: "danger",
        summary: "Admission queue saturation causing delayed rollout",
        state: "Mitigating",
        state_variant: Variant::Danger,
    },
    IncidentFixture {
        id: "INC-1044",
        cluster: "staging",
        service: "worker",
        severity: "info",
        summary: "Canary config drift detected and auto-corrected",
        state: "Resolved",
        state_variant: Variant::Success,
    },
];

const LATENCY_P95_POINTS: [MetricPoint; 8] = [
    MetricPoint {
        minute: 0,
        value: 94.0,
    },
    MetricPoint {
        minute: 5,
        value: 97.0,
    },
    MetricPoint {
        minute: 10,
        value: 102.0,
    },
    MetricPoint {
        minute: 15,
        value: 121.0,
    },
    MetricPoint {
        minute: 20,
        value: 113.0,
    },
    MetricPoint {
        minute: 25,
        value: 106.0,
    },
    MetricPoint {
        minute: 30,
        value: 98.0,
    },
    MetricPoint {
        minute: 35,
        value: 93.0,
    },
];

const ERROR_RATE_POINTS: [MetricPoint; 8] = [
    MetricPoint {
        minute: 0,
        value: 0.16,
    },
    MetricPoint {
        minute: 5,
        value: 0.21,
    },
    MetricPoint {
        minute: 10,
        value: 0.33,
    },
    MetricPoint {
        minute: 15,
        value: 0.71,
    },
    MetricPoint {
        minute: 20,
        value: 0.54,
    },
    MetricPoint {
        minute: 25,
        value: 0.39,
    },
    MetricPoint {
        minute: 30,
        value: 0.27,
    },
    MetricPoint {
        minute: 35,
        value: 0.19,
    },
];

const THROUGHPUT_POINTS: [MetricPoint; 8] = [
    MetricPoint {
        minute: 0,
        value: 9800.0,
    },
    MetricPoint {
        minute: 5,
        value: 10240.0,
    },
    MetricPoint {
        minute: 10,
        value: 10310.0,
    },
    MetricPoint {
        minute: 15,
        value: 9480.0,
    },
    MetricPoint {
        minute: 20,
        value: 9650.0,
    },
    MetricPoint {
        minute: 25,
        value: 10020.0,
    },
    MetricPoint {
        minute: 30,
        value: 10110.0,
    },
    MetricPoint {
        minute: 35,
        value: 10280.0,
    },
];

pub(super) const METRIC_SERIES: [MetricSeries; 3] = [
    MetricSeries {
        key: "latency_p95",
        label: "P95 latency",
        unit: "ms",
        variant: Variant::Warning,
        points: &LATENCY_P95_POINTS,
    },
    MetricSeries {
        key: "error_rate",
        label: "Error rate",
        unit: "%",
        variant: Variant::Danger,
        points: &ERROR_RATE_POINTS,
    },
    MetricSeries {
        key: "throughput",
        label: "Throughput",
        unit: "req/s",
        variant: Variant::Info,
        points: &THROUGHPUT_POINTS,
    },
];

pub(super) fn variant_palette() -> &'static [(&'static str, Variant)] {
    &VARIANTS
}

pub(super) fn for_each_variant(mut render: impl FnMut(&'static str, Variant)) {
    for &(label, variant) in variant_palette() {
        render(label, variant);
    }
}

pub(super) fn render_variant_gallery<'a, F>(ui: &mut UiRoot<'a>, mut render_item: F)
where
    F: FnMut(&mut UiRoot<'a>, &'static str, Variant),
{
    for &(label, variant) in variant_palette() {
        render_item(ui, label, variant);
    }
}

pub(super) fn cluster_by_index(index: usize) -> ClusterFixture {
    CLUSTERS[index.min(CLUSTERS.len() - 1)]
}

pub(super) fn service_by_index(index: usize) -> ServiceFixture {
    SERVICES[index.min(SERVICES.len() - 1)]
}
