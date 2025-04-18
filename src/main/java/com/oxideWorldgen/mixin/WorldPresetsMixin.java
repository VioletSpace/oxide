package com.oxideWorldgen.mixin;

import com.oxideWorldgen.Oxide;
import com.oxideWorldgen.gen.NoiseChunkGeneratorRust;
import net.minecraft.registry.RegistryKey;
import net.minecraft.world.dimension.DimensionOptions;
import net.minecraft.world.dimension.DimensionOptionsRegistryHolder;
import net.minecraft.world.gen.WorldPreset;
import net.minecraft.world.gen.WorldPresets;
import net.minecraft.world.gen.chunk.DebugChunkGenerator;
import net.minecraft.world.gen.chunk.FlatChunkGenerator;
import net.minecraft.world.gen.chunk.NoiseChunkGenerator;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

import java.util.Optional;

import static net.minecraft.world.gen.WorldPresets.DEBUG_ALL_BLOCK_STATES;
import static net.minecraft.world.gen.WorldPresets.FLAT;
import static net.minecraft.world.gen.WorldPresets.DEFAULT;

@Mixin(WorldPresets.class)
public class WorldPresetsMixin {
    @Inject(
            method = "getWorldPreset(Lnet/minecraft/world/dimension/DimensionOptionsRegistryHolder;)Ljava/util/Optional;",
            at = @At("HEAD"),
            cancellable = true
    )
    private static void onGetWorldPreset(DimensionOptionsRegistryHolder registry, CallbackInfoReturnable<Optional<RegistryKey<WorldPreset>>> cir) {
        Oxide.LOGGER.info("onGetWorldPreset");
        cir.setReturnValue(
                registry.getOrEmpty(DimensionOptions.OVERWORLD).flatMap(overworld -> {
                    return switch (overworld.chunkGenerator()) {
                        case FlatChunkGenerator flatChunkGenerator -> Optional.of(FLAT);
                        case DebugChunkGenerator debugChunkGenerator -> Optional.of(DEBUG_ALL_BLOCK_STATES);
                        case NoiseChunkGeneratorRust noiseChunkGeneratorRust -> Optional.of(DEFAULT);
                        default -> Optional.empty();
                    };
                })
        );
    }
}
