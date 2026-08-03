use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum IndexStep {
    All,
    IndexNodesCoords,
    StoreWays,
    Other,
}
