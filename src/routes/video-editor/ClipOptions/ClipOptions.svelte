<script lang="ts">
    import type { ComponentType } from 'svelte';
    import Basic from './Tabs/Basic.svelte';
    import Advanced from './Tabs/Advanced.svelte';
    import Share from './Tabs/Share.svelte';
    import { fly } from 'svelte/transition';
    import Stats from './Stats.svelte';

    import { invoke } from '@tauri-apps/api/core';
    import type { ClipCreationOptions } from '$lib/models/ClipCreationOptions';
    import {
        calculatedAudioBitrate,
        clipEnd,
        clipName,
        clipStart,
        finalBitrate,
        format,
        framerate,
        muteAudio,
        resolution,
        speed,
        validationErrors
    } from '$lib/stores/ClipOptionsStore';
    import { selectedVideo } from '$lib/stores/VideoEditorStore';
    import { goto } from '$app/navigation';

    let currentTab: ComponentType = Basic;

    let moveDirection = 0;

    const tabOrder = [Basic, Advanced, Share];

    function changeTab(to: ComponentType) {
        if (currentTab === to) return;
        moveDirection =
            (tabOrder.indexOf(currentTab) - tabOrder.indexOf(to)) * -1;
        currentTab = to;
    }

    async function createClip() {
        if (!$selectedVideo) {
            console.log('No video selected, this should not happen');
            return;
        }
        const name = $clipName + '.mp4';
        const exists = await invoke<boolean>('clip_exists', { file: name });
        let confirmed = true;
        if (exists) {
            confirmed = confirm(`Clip ${name} already exists, do you want to override it?`);
        }
        if (!confirmed) return;
        const options: ClipCreationOptions = {
            from: $selectedVideo,
            to: name,
            start: $clipStart,
            end: $clipEnd,
            video_bitrate: $finalBitrate,
            audio_bitrate: $calculatedAudioBitrate,
            framerate: $framerate,
            speed: $speed,
            resolution: $resolution,
            mute: $muteAudio,
            format: $format.name,
            preset: $format.preset
        };

        await invoke('create_clip', { options });

        await goto('/processing');
    }

</script>

<div class="px-3">
    <!--     <h2 class="text-2xl text-center mb-2">Create Clip</h2>
     -->
    <div class="tabs tabs-boxed flex-nowrap justify-center">
        <button
                class:tab-active={currentTab === Basic}
                on:click={() => { changeTab(Basic); }}
                class="tab flex-grow">Basic
        </button
        >
        <button
                class:tab-active={currentTab === Advanced}
                on:click={() => { changeTab(Advanced); }}
                class="tab flex-grow">Advanced
        </button
        >
        <button
                disabled={true}
                class:tab-active={currentTab === Share}
                on:click={() => { changeTab(Share); }}
                class="tab flex-grow">Sharing
        </button
        >
    </div>

    {#key currentTab}
        <div
                in:fly={{ x: 100 * moveDirection, duration: 200, delay: 200 }}
                out:fly={{ x: -100 * moveDirection, duration: 200 }}
        >
            <svelte:component this={currentTab}/>
            <button on:click={async () => { await createClip(); }} disabled={$validationErrors.length > 0}
                    class="btn w-full my-2 btn-primary"><i class="fa-solid fa-flag-checkered"></i> Create Clip!
            </button>
            <Stats/>
        </div>
    {/key}

</div>
