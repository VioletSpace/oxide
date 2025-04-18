package com.oxideWorldgen.mixin;

import com.mojang.serialization.MapCodec;
import com.oxideWorldgen.Oxide;
import com.oxideWorldgen.gen.NoiseChunkGeneratorRust;
import net.minecraft.registry.Registry;
import net.minecraft.world.gen.chunk.*;
import org.spongepowered.asm.mixin.*;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/*
* This mixin replaces the NoiseChunkGenerator provided by Minecraft with NoiseChunkGeneratorRust
* */
@Mixin(net.minecraft.world.gen.chunk.ChunkGenerators.class)
public class ChunkGeneratorsMixin {
    @Inject(
            method = "registerAndGetDefault(Lnet/minecraft/registry/Registry;)Lcom/mojang/serialization/MapCodec;",
            at = @At("HEAD"),
            cancellable = true
    )
    private static void registerAndGetDefaultRust(
            Registry<MapCodec<? extends ChunkGenerator>> registry, CallbackInfoReturnable<MapCodec> cir
    ) {
        Oxide.LOGGER.info("Replacing noise chunk generator");
        Registry.register(registry, "noise", NoiseChunkGeneratorRust.CODEC);
        Registry.register(registry, "flat", FlatChunkGenerator.CODEC);
        cir.setReturnValue(Registry.register(registry, "debug", DebugChunkGenerator.CODEC));
    }
}
