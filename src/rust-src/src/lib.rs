use std::collections::HashMap;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jint, jstring};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_oxideWorldgen_gen_NoiseChunkGeneratorRust_hello<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, input: JString<'local>) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(&input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello {}, from Rust!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_oxideWorldgen_gen_NoiseChunkGeneratorRust_getWorldHeightRust(
    mut env: JNIEnv,
    this: JObject,
) -> jint {
    let settings_obj = env
        .get_field(&this, "settings", "Lnet/minecraft/registry/entry/RegistryEntry;")
        .expect("Failed to get 'settings' field")
        .l()
        .expect("Expected settings to be an object");

    let settings_value = env
        .call_method(&settings_obj, "value", "()Ljava/lang/Object;", &[])
        .expect("Failed to call value()")
        .l()
        .unwrap();

    let shape_config = env
        .call_method(
            &settings_value,
            "generationShapeConfig",
            "()Lnet/minecraft/world/gen/chunk/GenerationShapeConfig;",
            &[],
        )
        .expect("Could not call generationShapeConfig()")
        .l()
        .expect("Expected GenerationShapeConfig");

    // 4. Call height() → int
    let height = env
        .call_method(&shape_config, "height", "()I", &[])
        .expect("Could not call height()")
        .i()
        .expect("Expected int from height()");

    height
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_oxideWorldgen_gen_NoiseChunkGeneratorRust_getSeaLevelRust(
    _env: JNIEnv,
    _this: JObject,
) -> jint {
    50
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_oxideWorldgen_gen_NoiseChunkGeneratorRust_populateNoiseRust<'local>(
    _env: JNIEnv<'local>,
    _this: JObject<'local>,
    chunk_noise_sampler_j: JObject<'local>,
    chunk_j: JObject<'local>,
    minimum_cell_y: jint,
    cell_height: jint
) -> JObject<'local> {
    let mut cns = ChunkNoiseSampler::from_java(chunk_noise_sampler_j);
    let mut chunk = Chunk::from_java(chunk_j);

    let i = chunk.pos.get_start_x();
    let j = chunk.pos.get_start_z();
    let aqs = &mut cns.aquifer_sampler;

    cns.sample_start_density();
    let mut mutable = BlockPos::new();
    let k = cns.horizontal_cell_block_count;
    let l = cns.vertical_cell_block_count;
    let m = 16 / k;
    let n = 16 / k;

    for o in 0..m {
        cns.sample_end_density(o);

        for p in 0..n {
            let q = chunk.count_vertical_sections() - 1;
            let chunk_section = &mut chunk.sections[q as usize];

            for r in (0..=(cell_height-1)).rev() {
                cns.on_sampled_cell_corners(r, p);
            }
        }
    }
    
    todo!()
}

struct Chunk {
    pos: ChunkPos,
    sections: Vec<ChunkSection>
}

struct ChunkSection {

}

impl Chunk {
    fn from_java(chunk_j: JObject) -> Chunk {
        todo!()
    }

    fn count_vertical_sections(&self) -> i32 {
        todo!()
    }
}

struct ChunkPos {
    x: i32,
    z: i32
}

impl ChunkPos {
    fn get_start_x(&self) -> i32 {
        Self::chunk_to_block_coord(self.x)
    }

    fn get_start_z(&self) -> i32 {
        Self::chunk_to_block_coord(self.z)
    }

    fn chunk_to_block_coord(c: i32) -> i32 {
        c << 4
    }
}

struct BlockPos {
    x: i32,
    y: i32,
    z: i32,
}

impl BlockPos {
    fn new() -> BlockPos {
        BlockPos { x: 0, y: 0, z: 0 }
    }
}

struct GenerationShapeConfig {
    
}

struct DensityInterpolator {
    start_density_buffer: Vec<Vec<f64>>,
    end_density_buffer: Vec<Vec<f64>>,
    x0y0z0: f64,
    x0y0z1: f64,
    x1y0z0: f64,
    x1y0z1: f64,
    x0y1z0: f64,
    x0y1z1: f64,
    x1y1z0: f64,
    x1y1z1: f64,
}

impl DensityInterpolator {
    fn fill(&mut self, start: bool, i: i32, applier: &EachApplier, cache: bool) {
        let ds = if start {
                &mut self.start_density_buffer[i as usize]
            } else {
                &mut self.end_density_buffer[i as usize]
            };
        if cache {
            
        }
        todo!()
    }

    fn on_sampled_cell_corners(&mut self, cell_y: i32, cell_z: i32) {
        self.x0y0z0 = self.start_density_buffer[cell_z as usize][cell_y as usize];
        self.x0y0z1 = self.start_density_buffer[cell_z as usize + 1][cell_y as usize];
        self.x1y0z0 = self.end_density_buffer[cell_z as usize][cell_y as usize];
        self.x1y0z1 = self.end_density_buffer[cell_z as usize + 1][cell_y as usize];
        self.x0y1z0 = self.start_density_buffer[cell_z as usize][cell_y as usize + 1];
        self.x0y1z1 = self.start_density_buffer[cell_z as usize + 1][cell_y as usize + 1];
        self.x1y1z0 = self.end_density_buffer[cell_z as usize][cell_y as usize + 1];
        self.x1y1z1 = self.end_density_buffer[cell_z as usize + 1][cell_y as usize + 1];
    }
}

struct DensityFunction {

}

struct CellCache {
    cache: i32,
}

impl CellCache {
    fn fill(cache: i32, applier: &EachApplier) {
        
    }
}

struct Long2IntMap {

}

struct AquiferSampler {

}

struct BlockStateSampler {

}

struct Blender {

}

struct BlendResult {

}

struct FlatCache {

}

struct Beardifying {
    
}

struct EachApplier {

}


struct ChunkNoiseSampler {
    generation_shape_config: GenerationShapeConfig,
    pub horizontal_cell_count: i32,
	pub vertical_cell_count: i32,
    pub minimum_cell_y: i32,
	start_cell_x: i32,
	start_cell_z: i32,
	pub start_biome_x: i32,
	pub start_biome_z: i32,
	pub interpolators: Vec<DensityInterpolator>,
	pub caches: Vec<CellCache>,
	actual_density_function_cache: HashMap<DensityFunction, DensityFunction>,
	surface_height_estimate_cache: Long2IntMap,
	aquifer_sampler: AquiferSampler,
	initial_density_without_jaggedness: DensityFunction,
	block_state_sampler: BlockStateSampler,
	blender: Blender,
	cached_blend_alpha_density_function: FlatCache,
	cached_blend_offset_density_function: FlatCache,
	beardifying: Beardifying,
	last_blending_column_pos: i64,
	last_blending_result: BlendResult,
	pub horizontal_biome_end: i32,
	pub horizontal_cell_block_count: i32,
	pub vertical_cell_block_count: i32,
	pub is_in_interpolation_loop: bool,
	pub is_sampling_for_caches: bool,
	start_block_x: i32,
	pub start_block_y: i32,
	start_block_z: i32,
	pub cell_block_x: i32,
	pub cell_block_y: i32,
	pub cell_block_z: i32,
	pub sample_unique_index: i64,
	pub cache_once_unique_index: i64,
	pub index: i32,

    interpolation_each_applier: EachApplier,
}

impl ChunkNoiseSampler {
    fn new(
        horizontal_cell_count: i32,
    ) -> ChunkNoiseSampler {

        todo!()
    }

    fn from_java(cns: JObject) -> ChunkNoiseSampler {
        todo!()
    }

    fn on_sampled_cell_corners(&mut self, cell_y: i32, cell_z: i32) {
        for dint in &mut self.interpolators {
            dint.on_sampled_cell_corners(cell_y, cell_z);
        }

        self.is_sampling_for_caches = true;
        self.start_block_y = (cell_y + self.minimum_cell_y) * self.vertical_cell_block_count;
        self.start_block_z = (cell_z + self.start_cell_z) * self.horizontal_cell_block_count;
        self.cache_once_unique_index += 1;

        for ccache in &self.caches {
            CellCache::fill(ccache.cache, &self.interpolation_each_applier);
        }

        self.cache_once_unique_index += 1;
        self.is_sampling_for_caches = false;
    }

    fn sample_density(&mut self, start: bool, cell_x: i32) {
        self.start_block_x = cell_x * self.horizontal_cell_block_count;
        self.cell_block_x = 0;

        for i in 0..=self.horizontal_cell_count {
            let j = self.start_cell_z + i;
            self.start_block_z = j * self.horizontal_cell_block_count;
            self.cell_block_z = 0;
            self.cache_once_unique_index += 1;

            for dint in &mut self.interpolators {
                dint.fill(start, i, &self.interpolation_each_applier, self.is_sampling_for_caches);
            }
        }

        self.cache_once_unique_index += 1;
    }

    fn sample_start_density(&mut self) {
        if self.is_in_interpolation_loop {
            panic!("Starting interpolation twice")
        } else {
            self.is_in_interpolation_loop = true;
            self.sample_unique_index = 0;
            self.sample_density(true, self.start_cell_x);
        }
    }

    fn sample_end_density(&mut self, cell_x: i32) {
        self.sample_density(false, self.start_cell_x + cell_x + 1);
    }

}