use pixi_build_types::{BinaryPackageSpec, SourcePackageLocationSpec, SourcePackageSpec};
use pixi_spec::{
    BinarySpec, DetailedSpec, MatchspecFields, SourceLocationSpec, UrlBinarySpec, UrlSourceSpec,
};
use rattler_conda_types::NamedChannelOrUrl;

/// Converts a [`SourcePackageSpec`] to a [`pixi_spec::SourceLocationSpec`].
pub fn from_source_spec_v1(source: SourcePackageSpec) -> SourceLocationSpec {
    let SourcePackageSpec {
        location,
        version,
        build,
        build_number,
        subdir,
        license,
    } = source;
    let mut location = from_source_package_location_spec(location);
    *location.matchspec_mut() = MatchspecFields {
        version,
        build,
        build_number,
        subdir,
        license,
        ..MatchspecFields::default()
    };
    location
}

pub fn from_source_package_location_spec(spec: SourcePackageLocationSpec) -> SourceLocationSpec {
    match spec {
        SourcePackageLocationSpec::Url(url) => SourceLocationSpec::Url(UrlSourceSpec::new(
            url.url,
            url.md5,
            url.sha256,
            url.subdirectory
                .and_then(|s| pixi_spec::Subdirectory::try_from(s).ok())
                .unwrap_or_default(),
        )),

        SourcePackageLocationSpec::Git(git) => SourceLocationSpec::Git(pixi_spec::GitSpec::new(
            git.git,
            git.rev.map(|r| match r {
                pixi_build_frontend::types::GitReference::Branch(b) => {
                    pixi_spec::GitReference::Branch(b)
                }
                pixi_build_frontend::types::GitReference::Tag(t) => pixi_spec::GitReference::Tag(t),
                pixi_build_frontend::types::GitReference::Rev(rev) => {
                    pixi_spec::GitReference::Rev(rev)
                }
                pixi_build_frontend::types::GitReference::DefaultBranch => {
                    pixi_spec::GitReference::DefaultBranch
                }
            }),
            git.subdirectory
                .and_then(|s| pixi_spec::Subdirectory::try_from(s).ok())
                .unwrap_or_default(),
        )),

        SourcePackageLocationSpec::Path(path) => {
            SourceLocationSpec::Path(pixi_spec::PathSourceSpec::new(path.path))
        }
    }
}

/// Converts a [`BinaryPackageSpec`] to a [`pixi_spec::BinarySpec`].
pub fn from_binary_spec_v1(spec: BinaryPackageSpec) -> pixi_spec::BinarySpec {
    match spec {
        BinaryPackageSpec {
            url: Some(url),
            sha256,
            md5,
            ..
        } => BinarySpec::Url(UrlBinarySpec { url, md5, sha256 }),
        BinaryPackageSpec {
            version: Some(version),
            build: None,
            build_number: None,
            file_name: None,
            channel: None,
            subdir: None,
            md5: None,
            sha256: None,
            license: None,
            condition: None,
            url: _,
        } => BinarySpec::Version(version),
        BinaryPackageSpec {
            version,
            build,
            build_number,
            file_name,
            channel,
            subdir,
            md5,
            sha256,
            license,
            condition,
            url: _,
        } => BinarySpec::DetailedVersion(Box::new(DetailedSpec {
            version,
            build,
            build_number,
            file_name,
            extras: None,
            flags: None,
            channel: channel.map(NamedChannelOrUrl::Url),
            subdir,
            license,
            license_family: None,
            condition,
            track_features: None,
            md5,
            sha256,
        })),
    }
}
