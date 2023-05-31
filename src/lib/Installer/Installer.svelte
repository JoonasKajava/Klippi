<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import MissingDependencies from "./MissingDependencies.svelte";
    import { fly, slide } from "svelte/transition";
    import Options from "./Steps/Options.svelte";
    import Downloading from "./Steps/Downloading.svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import { onDestroy } from "svelte";
    import Installing from "./Steps/Installing.svelte";
    import Verifying from "./Steps/Verifying.svelte";
    import Done from "./Steps/Done.svelte";
    import { ffmpeg_install_location } from "./InstallerStore";
    import VideoSelector from "../VideoSelector.svelte";
    import { current_page } from "../shared/AppStore";
    import type { StepChange } from "src/models/StepChange";
    import PanicRoom from "../PanicRoom/PanicRoom.svelte";

    let missing_dependencies: string[];

    let error: String | null;

    invoke("verify_dependencies")
        .then((missing: string[]) => {
            if (missing.length <= 0) {
                $current_page = VideoSelector;
            } else {
                missing_dependencies = missing;
            }
        })
        .catch((error_result: string) => {
            error = error_result;
            current_page.setWithProps<PanicRoom>(PanicRoom, {title: "Installer Ran Into An Error", error: error_result})
        });

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

    function install_dependencies() {
        invoke("install_dependencies", {
            path: $ffmpeg_install_location,
        }).catch((e: string) => {
            current_page.setWithProps<PanicRoom>(PanicRoom, {title: "Installer Ran Into An Error", error: e})
        });
    }
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
        if (current_step.name == "Options") install_dependencies();
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

                    {#if error}
                        <div
                            transition:slide
                            class="alert alert-error shadow-lg m-3"
                        >
                            <div>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="stroke-current flex-shrink-0 h-6 w-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    ><path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    /></svg
                                >
                                <span>{error}</span>
                            </div>
                        </div>
                    {/if}
                </div>
            </div>
        </div>
    {:else if !installer_inprogress && missing_dependencies}
        <div
            transition:fly={{ x: -100, duration: 200 }}
            on:outroend={() => (should_installer_start = true)}
        >
            <MissingDependencies
                onInstallClick={() => (installer_inprogress = true)}
                {missing_dependencies}
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
