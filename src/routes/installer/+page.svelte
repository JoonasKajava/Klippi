<script lang="ts">

    import MissingDependencies from './MissingDependencies.svelte';
    import { fly } from 'svelte/transition';
    import Downloading from './Steps/Downloading.svelte';
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
    import type { Event, UnlistenFn } from '@tauri-apps/api/event';
    import { onDestroy } from 'svelte';
    import Installing from './Steps/Installing.svelte';
    import Verifying from './Steps/Verifying.svelte';
    import Done from './Steps/Done.svelte';
    import type { StepChange } from '$lib/models/StepChange';
    import { missingDependencies } from '$lib/stores/InstallerStore';
const appWindow = getCurrentWebviewWindow()

    interface Step {
        name: string
        completed: boolean
        component?: ConstructorOfATypedSvelteComponent
    }

    let steps: Step[] = [
        {
            name: 'Downloading',
            completed: false,
            component: Downloading
        },
        {
            name: 'Installing',
            completed: false,
            component: Installing
        },
        {
            name: 'Verifying',
            completed: false,
            component: Verifying
        },
        {
            name: 'Done',
            completed: false,
            component: Done
        }
    ];

    let installerInProgress = false;
    let shouldInstallerStart = false;
    $: completed_steps = steps.filter((x) => x.completed);
    let currentStep: Step = steps[0];

    let unlisten: UnlistenFn;

    appWindow
        .listen('step_change', (event: Event<StepChange>) => {
            const find = steps.find(
                (x) => x.name === event.payload.previous_step
            );
            if (find) {
                find.completed = true;
            } else {
                console.error('Could not find step', event.payload.previous_step);
            }
            const nextStep = steps.find(
                (x) => x.name === event.payload.next_step
            );
            if (nextStep) {
                currentStep = nextStep;
            } else {
                console.error('Could not find step', event.payload.next_step);
            }
            steps = steps;
        })
        .then((handle) => {
            unlisten = handle;
        }).catch(console.error);

    onDestroy(() => {
        if (unlisten) unlisten();
    });

    function nextStep() {
        currentStep.completed = true;
        currentStep = steps[steps.indexOf(currentStep) + 1];
        steps = steps;
    }

    const stepTransitions = {
        in: { delay: 200, x: 100, duration: 200 },
        out: { x: -100, duration: 200 }
    };
</script>

<div class="overflow-hidden">
    {#if shouldInstallerStart}
        <div transition:fly={{ x: 100, duration: 200 }} class="hero">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-3xl mb-6">{currentStep.name}</h1>
                    <ul class="steps pb-4">
                        {#each Object.values(steps) as step}
                            <li
                                    class="step after:transition-all"
                                    class:step-primary={completed_steps.includes(
                                    step
                                )}
                                    class:step-warning={currentStep === step}
                            >
                                {step.name}
                            </li>
                        {/each}
                    </ul>
                    {#if currentStep.component}
                        <svelte:component
                                this={currentStep.component}
                                onNext={nextStep}
                                transitions={stepTransitions}
                        />
                    {/if}
                </div>
            </div>
        </div>
    {:else if !installerInProgress && $missingDependencies}
        <div
                transition:fly={{ x: -100, duration: 200 }}
                on:outroend={() => (shouldInstallerStart = true)}
        >
            <MissingDependencies
                    onInstallClick={() => (installerInProgress = true)}
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
