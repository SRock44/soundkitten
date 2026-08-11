<script lang="ts">
  import type { Location, Time, Weather } from '../types'
  import BeachScene from './locations/BeachScene.svelte'
  import CityScene from './locations/CityScene.svelte'
  import DesertScene from './locations/DesertScene.svelte'
  import FarmScene from './locations/FarmScene.svelte'
  import ForestScene from './locations/ForestScene.svelte'
  import HarborScene from './locations/HarborScene.svelte'
  import LondonScene from './locations/LondonScene.svelte'
  import MountainScene from './locations/MountainScene.svelte'
  import SuburbScene from './locations/SuburbScene.svelte'
  import TokyoScene from './locations/TokyoScene.svelte'
  import BeachLife from './life/BeachLife.svelte'
  import CityLife from './life/CityLife.svelte'
  import DesertLife from './life/DesertLife.svelte'
  import FarmLife from './life/FarmLife.svelte'
  import ForestLife from './life/ForestLife.svelte'
  import HarborLife from './life/HarborLife.svelte'
  import LondonLife from './life/LondonLife.svelte'
  import MountainLife from './life/MountainLife.svelte'
  import SuburbLife from './life/SuburbLife.svelte'
  import TokyoLife from './life/TokyoLife.svelte'

  let {
    location,
    weather,
    time,
  }: { location: Location; weather: Weather; time: Time } = $props()
</script>

{#if location === 'city'}
  <CityScene {weather} {time} />
  <CityLife {weather} />
{:else if location === 'tokyo'}
  <TokyoScene {weather} {time} />
  <TokyoLife {weather} />
{:else if location === 'london'}
  <LondonScene {weather} {time} />
  <LondonLife {weather} />
{:else if location === 'farm'}
  <FarmScene {weather} />
  <FarmLife {weather} />
{:else if location === 'forest'}
  <ForestScene {weather} />
  <ForestLife {weather} />
{:else if location === 'mountain'}
  <MountainScene {weather} />
  <MountainLife {weather} />
{:else if location === 'beach'}
  <BeachScene {weather} />
  <BeachLife {weather} />
{:else if location === 'harbor'}
  <HarborScene {weather} />
  <HarborLife {weather} />
{:else if location === 'desert'}
  <DesertScene {weather} />
  <DesertLife {weather} />
{:else}
  <SuburbScene {weather} />
  <SuburbLife {weather} />
{/if}
