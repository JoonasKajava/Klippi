<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import MissingDependencies from "./MissingDependencies.svelte";
    import { fly } from "svelte/transition";
    import Options from "./Steps/Options.svelte";
    import Downloading from "./Steps/Downloading.svelte";
    export let missing_dependencies: string[];

    interface Step {
        name: String;
        completed: boolean;
        component?: ConstructorOfATypedSvelteComponent;
    }

    let steps: Step[] = [
        {
            name: "Options",
            completed: false,
            component: Options,
        },
        {
            name: "Downloading",
            completed: false,
            component: Downloading
        },
        {
            name: "Installing",
            completed: false,
        },
        {
            name: "Verifying",
            completed: false,
        },
        {
            name: "Done",
            completed: false,
        },
    ];

    let installer_inprogress = false;
    let should_installer_start = false;
    $: completed_steps = steps.filter(x => x.completed);
    let current_step: Step = steps[0];

    function install_dependencies() {
        invoke("install_dependencies", {
            path: "D:\\Downloads\\ffmpeg"
        });
    }

    function next_step() {
        if(current_step.name == "Options") install_dependencies();
        current_step.completed = true;
        current_step = steps[steps.indexOf(current_step) + 1];
        steps = steps;
    }
</script>

{#if should_installer_start}
    <div transition:fly={{ y: -100, duration: 200 }} class="hero">
        <div class="hero-content text-center">
            <div class="max-w-md">
                <h1 class="text-3xl mb-6">{current_step.name}</h1>
                <ul class="steps pb-4">
                    {#each Object.values(steps) as step}
                        <li
                            class="step"
                            class:step-primary={completed_steps.indexOf(step) > -1}
                            class:step-warning={current_step == step}
                        >
                            {step.name}
                        </li>
                    {/each}
                </ul>
                {#if current_step.component}
                    <svelte:component
                        this={current_step.component}
                        onNext={next_step}
                    />
                {/if}
            </div>
        </div>
    </div>
{:else if !installer_inprogress}
    <div
        transition:fly={{ y: 100, duration: 200 }}
        on:outroend={() => (should_installer_start = true)}
    >
        <MissingDependencies
            onInstallClick={() => installer_inprogress = true}
            {missing_dependencies}
        />
    </div>
{/if}
