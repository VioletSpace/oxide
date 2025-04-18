package com.oxideWorldgen.mixin;

import com.mojang.datafixers.DataFixer;
import com.oxideWorldgen.Oxide;
import com.oxideWorldgen.gen.NoiseChunkGeneratorRust;
import net.minecraft.registry.DynamicRegistryManager;
import net.minecraft.registry.RegistryKeys;
import net.minecraft.server.WorldGenerationProgressListener;
import net.minecraft.server.world.ChunkTicketManager;
import net.minecraft.server.world.ServerChunkLoadingManager;
import net.minecraft.server.world.ServerWorld;
import net.minecraft.structure.StructureTemplateManager;
import net.minecraft.util.thread.ThreadExecutor;
import net.minecraft.world.chunk.ChunkProvider;
import net.minecraft.world.chunk.ChunkStatusChangeListener;
import net.minecraft.world.gen.chunk.ChunkGenerator;
import net.minecraft.world.gen.chunk.ChunkGeneratorSettings;
import net.minecraft.world.gen.noise.NoiseConfig;
import net.minecraft.world.level.storage.LevelStorage;
import org.spongepowered.asm.mixin.Final;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Mutable;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import java.util.concurrent.Executor;
import java.util.function.Supplier;

@Mixin(ServerChunkLoadingManager.class)
public class ServerChunkLoadingManagerMixin {
    @Shadow @Final @Mutable
    private NoiseConfig noiseConfig;

    @Inject(
            method = "<init>",
            at = @At("TAIL")
    )
    private void onCtorTail(
            ServerWorld world, LevelStorage.Session session, DataFixer dataFixer, StructureTemplateManager structureTemplateManager, Executor executor, ThreadExecutor mainThreadExecutor, ChunkProvider chunkProvider, ChunkGenerator chunkGenerator, WorldGenerationProgressListener worldGenerationProgressListener, ChunkStatusChangeListener chunkStatusChangeListener, Supplier persistentStateManagerFactory, ChunkTicketManager ticketManager, int viewDistance, boolean dsync, CallbackInfo ci
    ) {
        Oxide.LOGGER.info("Attempting ServerChunkLoadingManager noiseConfig override");
        DynamicRegistryManager dynamicRegistryManager = world.getRegistryManager();
        long l = world.getSeed();
        if (chunkGenerator instanceof NoiseChunkGeneratorRust noiseChunkGeneratorRust) {
            this.noiseConfig = NoiseConfig.create(noiseChunkGeneratorRust.getSettings().value(), dynamicRegistryManager.getOrThrow(RegistryKeys.NOISE_PARAMETERS), l);
            Oxide.LOGGER.info("noiseConfig successfully overridden");
        } else {
            this.noiseConfig = NoiseConfig.create(ChunkGeneratorSettings.createMissingSettings(), dynamicRegistryManager.getOrThrow(RegistryKeys.NOISE_PARAMETERS), l);
        }
    }
}
