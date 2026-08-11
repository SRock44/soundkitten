import type { Location, Time, Weather } from './types'

/** Brand fills sampled from the soundkitten logo */
export const BRAND = {
  orange: '#FD6309',
  headphones: '#3F63A1',
} as const

export type SkyPalette = {
  top: string
  upper: string
  mid: string
  lower: string
  bottom: string
  sun: string
  sunGlow: string
  accent: string
  cloud: string
  cloudShade: string
  horizon: string
}

export const TIME_SKY: Record<Time, SkyPalette> = {
  sunrise: {
    top: '#3A5A98',
    upper: '#6A7EB8',
    mid: '#FF8A68',
    lower: '#FFB878',
    bottom: '#FFE8C0',
    sun: '#FFC040',
    sunGlow: '#FFD898',
    accent: '#FFF0D8',
    cloud: '#FFE8D8',
    cloudShade: '#E8A890',
    horizon: '#FF9A58',
  },
  day: {
    top: '#3A92D8',
    upper: '#5BB0E8',
    mid: '#8ED0F4',
    lower: '#B6E4FA',
    bottom: '#D4F0FC',
    sun: '#FFE566',
    sunGlow: '#FFF3B0',
    accent: '#E8F6FF',
    cloud: '#FFFFFF',
    cloudShade: '#D8ECF8',
    horizon: '#C8E8F8',
  },
  sunset: {
    top: '#141838',
    upper: '#2A2868',
    mid: '#E04858',
    lower: '#FF7840',
    bottom: '#FFB870',
    sun: '#FF7A28',
    sunGlow: '#FFA858',
    accent: '#FFD0A0',
    cloud: '#E89888',
    cloudShade: '#6A3868',
    horizon: '#FF8A40',
  },
  night: {
    top: '#04060F',
    upper: '#080E1E',
    mid: '#101828',
    lower: '#182038',
    bottom: '#1E2848',
    sun: '#F4EED8',
    sunGlow: '#A8B8E0',
    accent: '#C8D4F0',
    cloud: '#1A2438',
    cloudShade: '#0E1424',
    horizon: '#3A5080',
  },
}

/** Seasonal particle accents beyond the primary WEATHER_PARTICLE swatch */
export const FALL_LEAF_COLORS = [
  '#E8A040',
  '#D47830',
  '#C45A28',
  '#A83228',
  '#B84820',
  '#E07030',
  '#C86838',
] as const

export const SPRING_PETAL_COLORS = [
  '#F0A0C8',
  '#FFB8D8',
  '#F8C8E0',
  '#E890B8',
  '#FFD0E8',
] as const

export const SUMMER_MOTE_COLORS = [
  '#FFE8A0',
  '#FFF6C8',
  '#FFD070',
  '#FFF0B8',
] as const

export type GroundPalette = {
  far: string
  mid: string
  near: string
  detail: string
  accent: string
  path: string
}

const LOCATION_BASE: Record<Location, GroundPalette> = {
  city: {
    far: '#4A5568',
    mid: '#5B6B7C',
    near: '#3D4A57',
    detail: '#2C3640',
    accent: '#7A8B9C',
    path: '#5A6570',
  },
  tokyo: {
    far: '#3A3A58',
    mid: '#4A4A6A',
    near: '#2A2A3A',
    detail: '#1A1A28',
    accent: '#E07090',
    path: '#4A4A58',
  },
  london: {
    far: '#6A7888',
    mid: '#8A6A58',
    near: '#5A6870',
    detail: '#3A4850',
    accent: '#C84848',
    path: '#5A6068',
  },
  farm: {
    far: '#6B8F4E',
    mid: '#8BB85C',
    near: '#5E7A3E',
    detail: '#C4A574',
    accent: '#E8D4A8',
    path: '#A89068',
  },
  forest: {
    far: '#2F5D3A',
    mid: '#3E7A4A',
    near: '#245032',
    detail: '#1A3A24',
    accent: '#5A9A62',
    path: '#3A4A32',
  },
  mountain: {
    far: '#6B7B8E',
    mid: '#8A9AAB',
    near: '#4E5E6E',
    detail: '#3A4856',
    accent: '#A8B8C8',
    path: '#6A7A6A',
  },
  beach: {
    far: '#5AB0C8',
    mid: '#E8D0A0',
    near: '#D4C090',
    detail: '#8A7A58',
    accent: '#F0E8C8',
    path: '#C8B888',
  },
  harbor: {
    far: '#4A6A7A',
    mid: '#5A7A88',
    near: '#3A5A68',
    detail: '#2A3A48',
    accent: '#8AA8B8',
    path: '#5A6870',
  },
  desert: {
    far: '#C8A070',
    mid: '#D4B888',
    near: '#B89060',
    detail: '#8A6A40',
    accent: '#E8D0A0',
    path: '#A87848',
  },
  suburb: {
    far: '#7AAA6A',
    mid: '#8ABA78',
    near: '#5A8A4A',
    detail: '#6A7A88',
    accent: '#C8A878',
    path: '#8A8A88',
  },
}

const URBAN = new Set<Location>(['city', 'tokyo', 'london'])

function urbanSeason(b: GroundPalette, weather: Weather): GroundPalette {
  switch (weather) {
    case 'fall':
      return {
        far: mixToward(b.far, '#6A5048', 0.28),
        mid: mixToward(b.mid, '#7A5848', 0.22),
        near: mixToward(b.near, '#4A4038', 0.2),
        detail: mixToward(b.detail, '#3A3230', 0.18),
        accent: '#E07830',
        path: mixToward(b.path, '#5A5048', 0.25),
      }
    case 'winter':
      return {
        far: mixToward(b.far, '#D0DCE8', 0.45),
        mid: mixToward(b.mid, '#E0EAF4', 0.4),
        near: mixToward(b.near, '#C0D0E0', 0.38),
        detail: mixToward(b.detail, '#A0B4C8', 0.3),
        accent: '#FFFFFF',
        path: mixToward(b.path, '#D8E4F0', 0.5),
      }
    case 'spring':
      return {
        far: mixToward(b.far, '#6A7888', 0.12),
        mid: mixToward(b.mid, '#7A8A98', 0.1),
        near: mixToward(b.near, '#4A5A48', 0.22),
        detail: mixToward(b.detail, '#3A4840', 0.15),
        accent: '#F0A0C8',
        path: mixToward(b.path, '#5A6860', 0.18),
      }
    case 'summer':
      return {
        far: mixToward(b.far, '#5A6878', 0.1),
        mid: mixToward(b.mid, '#6A7A88', 0.12),
        near: mixToward(b.near, '#4A5850', 0.18),
        detail: mixToward(b.detail, '#3A4840', 0.12),
        accent: '#F0C848',
        path: mixToward(b.path, '#5A6058', 0.15),
      }
  }
}

function beachSeason(b: GroundPalette, weather: Weather): GroundPalette {
  switch (weather) {
    case 'fall':
      return {
        far: mixToward(b.far, '#6A9AB0', 0.25),
        mid: mixToward(b.mid, '#C8A878', 0.35),
        near: mixToward(b.near, '#B89870', 0.3),
        detail: mixToward(b.detail, '#7A6848', 0.25),
        accent: '#E07030',
        path: mixToward(b.path, '#A88860', 0.3),
      }
    case 'winter':
      return {
        far: mixToward(b.far, '#A0C0D0', 0.4),
        mid: mixToward(b.mid, '#E0E4E8', 0.45),
        near: mixToward(b.near, '#D0D4D8', 0.4),
        detail: mixToward(b.detail, '#A0A8B0', 0.35),
        accent: '#FFFFFF',
        path: mixToward(b.path, '#C8CCD0', 0.4),
      }
    case 'spring':
      return {
        far: mixToward(b.far, '#6AC0D8', 0.2),
        mid: mixToward(b.mid, '#F0D8A8', 0.15),
        near: mixToward(b.near, '#6AAA68', 0.2),
        detail: mixToward(b.detail, '#F090B8', 0.2),
        accent: '#FFB0D0',
        path: mixToward(b.path, '#D4C090', 0.15),
      }
    case 'summer':
      return {
        far: mixToward(b.far, '#40A8C8', 0.25),
        mid: mixToward(b.mid, '#F0E0B0', 0.2),
        near: mixToward(b.near, '#E8D498', 0.15),
        detail: mixToward(b.detail, '#F0C848', 0.2),
        accent: '#FFE060',
        path: mixToward(b.path, '#E0D0A0', 0.15),
      }
  }
}

function desertSeason(b: GroundPalette, weather: Weather): GroundPalette {
  switch (weather) {
    case 'fall':
      return {
        far: mixToward(b.far, '#C87848', 0.35),
        mid: mixToward(b.mid, '#D49858', 0.3),
        near: mixToward(b.near, '#A86838', 0.28),
        detail: mixToward(b.detail, '#8A4828', 0.25),
        accent: '#F0A848',
        path: mixToward(b.path, '#986838', 0.25),
      }
    case 'winter':
      return {
        far: mixToward(b.far, '#D0C8B8', 0.4),
        mid: mixToward(b.mid, '#E0D8C8', 0.35),
        near: mixToward(b.near, '#C0B8A8', 0.35),
        detail: mixToward(b.detail, '#A09888', 0.3),
        accent: '#F0F0F0',
        path: mixToward(b.path, '#B0A898', 0.3),
      }
    case 'spring':
      return {
        far: mixToward(b.far, '#A8B878', 0.25),
        mid: mixToward(b.mid, '#C8C888', 0.2),
        near: mixToward(b.near, '#88A060', 0.28),
        detail: mixToward(b.detail, '#E090B0', 0.18),
        accent: '#D0E890',
        path: mixToward(b.path, '#A88858', 0.15),
      }
    case 'summer':
      return {
        far: mixToward(b.far, '#E0B070', 0.3),
        mid: mixToward(b.mid, '#E8C888', 0.25),
        near: mixToward(b.near, '#C89850', 0.22),
        detail: mixToward(b.detail, '#A87038', 0.2),
        accent: '#FFE060',
        path: mixToward(b.path, '#B88040', 0.2),
      }
  }
}

function harborSeason(b: GroundPalette, weather: Weather): GroundPalette {
  switch (weather) {
    case 'fall':
      return {
        far: mixToward(b.far, '#6A7A78', 0.25),
        mid: mixToward(b.mid, '#8A7A68', 0.28),
        near: mixToward(b.near, '#5A6A60', 0.22),
        detail: mixToward(b.detail, '#4A4A40', 0.2),
        accent: '#E07830',
        path: mixToward(b.path, '#6A6860', 0.25),
      }
    case 'winter':
      return {
        far: mixToward(b.far, '#C0D0D8', 0.5),
        mid: mixToward(b.mid, '#D0E0E8', 0.45),
        near: mixToward(b.near, '#B0C0C8', 0.42),
        detail: mixToward(b.detail, '#90A0B0', 0.35),
        accent: '#FFFFFF',
        path: mixToward(b.path, '#C8D4DC', 0.45),
      }
    case 'spring':
      return {
        far: mixToward(b.far, '#5A9A98', 0.25),
        mid: mixToward(b.mid, '#6AAA88', 0.22),
        near: mixToward(b.near, '#4A8A70', 0.2),
        detail: mixToward(b.detail, '#F090B8', 0.15),
        accent: '#FFB0D0',
        path: mixToward(b.path, '#6A8078', 0.18),
      }
    case 'summer':
      return {
        far: mixToward(b.far, '#3A8AA0', 0.28),
        mid: mixToward(b.mid, '#4A9AB0', 0.22),
        near: mixToward(b.near, '#2A7A88', 0.2),
        detail: mixToward(b.detail, '#F0C848', 0.15),
        accent: '#FFE060',
        path: mixToward(b.path, '#5A7878', 0.15),
      }
  }
}

/** Season tints applied on top of location bases */
export function groundFor(location: Location, weather: Weather): GroundPalette {
  const b = LOCATION_BASE[location]
  if (URBAN.has(location)) return urbanSeason(b, weather)
  if (location === 'beach') return beachSeason(b, weather)
  if (location === 'desert') return desertSeason(b, weather)
  if (location === 'harbor') return harborSeason(b, weather)

  switch (weather) {
    case 'fall':
      return {
        far: mixToward(b.far, '#C45A2A', 0.48),
        mid: mixToward(b.mid, '#E07830', 0.52),
        near: mixToward(b.near, '#8B3A18', 0.42),
        detail: mixToward(b.detail, '#A84820', 0.38),
        accent: '#F0A848',
        path: mixToward(b.path, '#8A5A28', 0.4),
      }
    case 'winter':
      return {
        far: mixToward(b.far, '#E0ECF8', 0.62),
        mid: mixToward(b.mid, '#F0F6FC', 0.58),
        near: mixToward(b.near, '#D0E0F0', 0.52),
        detail: mixToward(b.detail, '#B0C4D8', 0.42),
        accent: '#FFFFFF',
        path: mixToward(b.path, '#E0EAF4', 0.58),
      }
    case 'spring':
      return {
        far: mixToward(b.far, '#6AD868', 0.45),
        mid: mixToward(b.mid, '#98E878', 0.5),
        near: mixToward(b.near, '#48B850', 0.4),
        detail: mixToward(b.detail, '#F090B8', 0.32),
        accent: '#FFB0D0',
        path: mixToward(b.path, '#88C068', 0.35),
      }
    case 'summer':
      return {
        far: mixToward(b.far, '#58C048', 0.38),
        mid: mixToward(b.mid, '#88D850', 0.42),
        near: mixToward(b.near, '#3A9830', 0.32),
        detail: mixToward(b.detail, '#F0C848', 0.28),
        accent: '#FFE060',
        path: mixToward(b.path, '#D4B858', 0.3),
      }
  }
}

function mixToward(hex: string, toward: string, t: number): string {
  const a = parseHex(hex)
  const b = parseHex(toward)
  const m = (i: number) => Math.round(a[i] + (b[i] - a[i]) * t)
  return `#${[m(0), m(1), m(2)].map((n) => n.toString(16).padStart(2, '0')).join('')}`
}

function parseHex(hex: string): [number, number, number] {
  const h = hex.replace('#', '')
  return [
    parseInt(h.slice(0, 2), 16),
    parseInt(h.slice(2, 4), 16),
    parseInt(h.slice(4, 6), 16),
  ]
}

export const WEATHER_PARTICLE: Record<Weather, string> = {
  fall: '#E07030',
  winter: '#F4F8FC',
  spring: '#F0A0C8',
  summer: 'transparent',
}
