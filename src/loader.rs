use crate::{anim::AsepriteAnimation, Aseprite, error};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, RenderAssetUsages},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat}, text,
};
use bevy_aseprite_reader as reader;


#[derive(Debug, Default)]
pub struct AsepriteLoader;

impl AssetLoader for AsepriteLoader {
    type Asset = Aseprite;
    type Settings = ();
    type Error = error::AsepriteLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        debug!("Loading aseprite at {:?}", load_context.path());

        let mut buffer = vec![];
        let _ = reader.read_to_end(&mut buffer).await?;
        let data = Some(reader::Aseprite::from_bytes(buffer)?);

        Ok(Aseprite {
            data,
            info: None,
            frame_to_idx: vec![],
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}

pub(crate) fn process_load(
    mut asset_events: EventReader<AssetEvent<Aseprite>>,
    mut aseprites: ResMut<Assets<Aseprite>>,
    mut images: ResMut<Assets<Image>>,
) {
    asset_events.read().for_each(|event| {
        if let AssetEvent::Added { id } | AssetEvent::Modified { id } = event {

            let ase = match aseprites.get_mut(*id) {
                Some(ase) => ase,
                None => {
                    error!("Aseprite handle doesn't hold anything?");
                    return;
                }
            };
            let data = match ase.data.take() {
                Some(data) => data,
                None => {
                    error!("Ase data is empty");
                    return;
                }
            };

            // Build out texture atlas
            let frames = data.frames();
            let ase_images = frames
                .get_for(&(0..frames.count() as u16))
                .get_images()
                .unwrap();

            let mut frame_handles = vec![];
            let mut atlas = TextureAtlasBuilder::default();
            let mut textures = Vec::new();
            for (idx, image) in ase_images.into_iter().enumerate() {
                let texture = Image::new(
                    Extent3d {
                        width: image.width(),
                        height: image.height(),
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    image.into_raw(),
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::RENDER_WORLD
                );
                textures.push((idx, texture));
            }
            for (idx, texture) in textures.iter() {
                let _label = format!("Frame{}", idx);
                let texture_handle = images.add(texture.clone());
                frame_handles.push(texture_handle.clone_weak());
                
                atlas.add_texture(Some(texture_handle.id()), &texture);
            }

            let (_, atlas_source, _) = match atlas.build() {
                Ok(atlas) => atlas,
                Err(err) => {
                    error!("{:?}", err);
                    return;
                }
            };
            for handle in frame_handles {
                let atlas_idx = atlas_source.texture_index(&handle).unwrap();
                ase.frame_to_idx.push(atlas_idx);
            }
            ase.info = Some(data.into());
        }
    });
}
