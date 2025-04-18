#![allow(unused)]
use std::collections::HashMap;
use jni::JNIEnv;
use jni::objects::{JObject, JValueGen};
use jni::sys::jint;


#[unsafe(no_mangle)]
pub extern "system" fn Java_com_oxideWorldgen_gen_NoiseChunkGeneratorRust_populateNoiseRust<'local>(
    mut env: JNIEnv<'local>,
    _this: JObject<'local>,
    chunk_noise_sampler_j: JObject<'local>,
    chunk_j: JObject<'local>,
    minimum_cell_y: jint,
    cell_height: jint
) {
    let chunk: Chunk<'_> = Chunk::from_java(chunk_j);

    let mutable: BlockPos<'_> = BlockPos::new(&mut env);//BlockPos { jobj: obj };
    let block_state: BlockState<'_> = BlockState::default(&mut env);

    for x in 0..16 {
        for z in 0..16 {
            mutable.set(&mut env, x, 0, z);
            chunk.set_block_state(&mutable, &block_state, &mut env);
        }
    }
}

struct Chunk<'a> {
    jobj: JObject<'a>,
}

impl Chunk<'_> {
    fn from_java<'a>(chunk_j: JObject<'a>) -> Chunk<'a> {
        Chunk { 
            jobj: chunk_j,
        }
    }

    fn set_block_state(&self, pos: &BlockPos, state: &BlockState, env: &mut JNIEnv) {
        let _ = env.call_method(
            &self.jobj,
             "setBlockState",
             "(Lnet/minecraft/util/math/BlockPos;Lnet/minecraft/block/BlockState;)Lnet/minecraft/block/BlockState;",
             &[JValueGen::Object(&pos.jobj), JValueGen::Object(&state.jobj)]
            )
            .expect("update");
    }

    fn count_vertical_sections(&self) -> i32 {
        todo!()
    }

    fn get_pos(&self) -> ChunkPos {
        todo!()
    }

    fn get_section(&self, i: i32) -> ChunkSection {
        todo!()
    }
}

struct BlockPos<'a> {
    jobj: JObject<'a>,
}

impl BlockPos<'_> {
    fn new<'a>(env: &mut JNIEnv<'a>) -> BlockPos<'a> {
        let class_path = "net/minecraft/util/math/BlockPos$Mutable";

        let class = env.find_class(class_path).expect("1");

        let ctor_sig = "()V";
        let obj = env.new_object(class, ctor_sig, &[]).expect("msg");
        assert!(env.is_instance_of(&obj, "net/minecraft/util/math/BlockPos$Mutable").unwrap());
        BlockPos { 
            jobj: obj,
        }
    }

    fn set(&self, env: &mut JNIEnv,  x: i32, y: i32, z: i32) {
        let _ = env.call_method(
            &self.jobj,
             "set",
             "(III)Lnet/minecraft/util/math/BlockPos$Mutable;",
             &[JValueGen::Int(x), JValueGen::Int(y), JValueGen::Int(z)]
            )
            .expect("update");
    }
}

struct BlockState<'a> {
    jobj: JObject<'a>,
}

impl BlockState<'_> {
    fn default<'a>(env: &mut JNIEnv<'a>) -> BlockState<'a> {
        let block_class = env.find_class("net/minecraft/block/Blocks").expect("block_class");

        let stone_block = env.get_static_field(block_class, "STONE", "Lnet/minecraft/block/Block;").expect("msg").l().expect("msg");

        let state = env.call_method(stone_block, "getDefaultState", "()Lnet/minecraft/block/BlockState;", &[]).expect("msg").l().expect("msg");
        assert!(env.is_instance_of(&state, "net/minecraft/block/BlockState").unwrap());
        BlockState { 
            jobj: state,
        }
    }
}

struct ChunkSection {

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