<script lang="ts">

    import MissingDependencies from "./MissingDependencies.svelte";
    import { fly } from "svelte/transition";
    import Options from "./Steps/Options.svelte";
    import Downloading from "./Steps/Downloading.svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";
    import Installing from "./Steps/Installing.svelte";
    import Verifying from "./Steps/Verifying.svelte";
    import Done from "./Steps/Done.svelte";
    import type { StepChange } from "$lib/models/StepChange";
    import { missing_dependencies } from "$lib/stores/InstallerStore";


    interface Step {
        name: String;
        completed: boolean;
        component?: ConstructorOfATypedSvelteComponent;
    }

    let steps: Step[] = [
        {
            name: "Downloading",
            completed: false,
            component: Downloading,
        },
        {
            name: "Installing",
            completed: false,
            component: Installing,
        },
        {
            name: "Verifying",
            completed: false,
            component: Verifying,
        },
        {
            name: "Done",
            completed: false,
            component: Done,
        },
    ];

    let installer_inprogress = false;
    let should_installer_start = false;
    $: completed_steps = steps.filter((x) => x.completed);
    let current_step: Step = steps[0];


    let unlisten: UnlistenFn;

    appWindow
        .listen("step_change", (event: Event<StepChange>) => {
            steps.find(
                (x) => x.name === event.payload.previous_step
            ).completed = true;
            current_step = steps.find(
                (x) => x.name === event.payload.next_step
            );
            steps = steps;
        })
        .then((handle) => {
            unlisten = handle;
        });

    onDestroy(() => {
        if (unlisten) unlisten();
    });

    function next_step() {
        current_step.completed = true;
        current_step = steps[steps.indexOf(current_step) + 1];
        steps = steps;
    }

    let step_transitions = {
        in: { delay: 200, x: 100, duration: 200 },
        out: { x: -100, duration: 200 },
    };
</script>

<div class="overflow-hidden">
    {#if should_installer_start}
        <div transition:fly={{ x: 100, duration: 200 }} class="hero">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-3xl mb-6">{current_step.name}</h1>
                    <ul class="steps pb-4">
                        {#each Object.values(steps) as step}
                            <li
                                class="step after:transition-all"
                                class:step-primary={completed_steps.indexOf(
                                    step
                                ) > -1}
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
                            transitions={step_transitions}
                        />
                    {/if}
                </div>
            </div>
        </div>
    {:else if !installer_inprogress && $missing_dependencies}
        <div
            transition:fly={{ x: -100, duration: 200 }}
            on:outroend={() => (should_installer_start = true)}
        >
            <MissingDependencies
                onInstallClick={() => (installer_inprogress = true)}
            />
        </div>
    {/if}
</div>

<style lang="scss">
    .step-primary + .step-warning {
        &::before {
            background-image: linear-gradient(
                to right,
                hsl(var(--p) / var(--tw-bg-opacity)),
                hsl(var(--wa) / var(--tw-bg-opacity))
            );
        }
    }
</style>
