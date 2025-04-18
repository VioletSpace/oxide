package com.oxideWorldgen.mixin;

import net.minecraft.block.BlockState;
import net.minecraft.world.biome.source.util.MultiNoiseUtil;
import net.minecraft.world.gen.chunk.ChunkNoiseSampler;
import net.minecraft.world.gen.noise.NoiseRouter;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Invoker;

import java.util.List;

/*
* This mixin allows access to MultiNoiseUtil.MultiNoiseSampler
* */
@Mixin(ChunkNoiseSampler.class)
public interface ChunkNoiseSamplerMixin {
    @Invoker("createMultiNoiseSampler")
     MultiNoiseUtil.MultiNoiseSampler callCreateMultiNoiseSampler(
            NoiseRouter router,
            List<MultiNoiseUtil.NoiseHypercube> parameters
    );

    @Invoker("sampleBlockState")
    BlockState callSampleBlockState();

    @Invoker("getHorizontalCellBlockCount")
    int callGetHorizontalCellBlockCount();

    @Invoker("getVerticalCellBlockCount")
    int callGetVerticalCellBlockCount();
}
