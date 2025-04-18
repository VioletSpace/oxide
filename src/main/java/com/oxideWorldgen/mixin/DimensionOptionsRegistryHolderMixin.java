package com.oxideWorldgen.mixin;

import com.oxideWorldgen.Oxide;
import com.oxideWorldgen.gen.NoiseChunkGeneratorRust;
import net.minecraft.world.biome.source.MultiNoiseBiomeSource;
import net.minecraft.world.biome.source.MultiNoiseBiomeSourceParameterLists;
import net.minecraft.world.biome.source.TheEndBiomeSource;
import net.minecraft.world.dimension.DimensionOptions;
import net.minecraft.world.dimension.DimensionOptionsRegistryHolder;
import net.minecraft.world.dimension.DimensionTypes;
import net.minecraft.world.gen.chunk.ChunkGeneratorSettings;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/*
* This Mixin maxes sure that the vanilla checks for Nether and End use the correct (overwritten) generator
* */
@Mixin(DimensionOptionsRegistryHolder.class)
public class DimensionOptionsRegistryHolderMixin {
    @Inject(
            method = "isNetherVanilla(Lnet/minecraft/world/dimension/DimensionOptions;)Z",
            at = @At("HEAD"),
            cancellable = true
    )
    private static void onIsNetherVanilla(DimensionOptions dimensionOptions, CallbackInfoReturnable<Boolean> cir) {
        cir.setReturnValue(
                dimensionOptions.dimensionTypeEntry().matchesKey(DimensionTypes.THE_NETHER)
                        && dimensionOptions.chunkGenerator() instanceof NoiseChunkGeneratorRust noiseChunkGeneratorRust
                        && noiseChunkGeneratorRust.matchesSettings(ChunkGeneratorSettings.NETHER)
                        && noiseChunkGeneratorRust.getBiomeSource() instanceof MultiNoiseBiomeSource multiNoiseBiomeSource
                        && multiNoiseBiomeSource.matchesInstance(MultiNoiseBiomeSourceParameterLists.NETHER)
        );
        Oxide.LOGGER.info("Fixed Nether vanilla check");
    }

    @Inject(
            method = "isTheEndVanilla(Lnet/minecraft/world/dimension/DimensionOptions;)Z",
            at = @At("HEAD"),
            cancellable = true
    )
    private static void onIsTheEndVanilla(DimensionOptions dimensionOptions, CallbackInfoReturnable<Boolean> cir) {
        cir.setReturnValue(
                dimensionOptions.dimensionTypeEntry().matchesKey(DimensionTypes.THE_END)
                        && dimensionOptions.chunkGenerator() instanceof NoiseChunkGeneratorRust noiseChunkGeneratorRust
                        && noiseChunkGeneratorRust.matchesSettings(ChunkGeneratorSettings.END)
                        && noiseChunkGeneratorRust.getBiomeSource() instanceof TheEndBiomeSource
        );
        Oxide.LOGGER.info("Fixed The End vanilla check");
    }
}
