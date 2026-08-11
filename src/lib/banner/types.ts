export const TIMES = ['sunrise', 'day', 'sunset', 'night'] as const
export const WEATHERS = ['fall', 'winter', 'spring', 'summer'] as const
export const LOCATIONS = [
  'city',
  'tokyo',
  'london',
  'farm',
  'forest',
  'mountain',
  'beach',
  'harbor',
  'desert',
  'suburb',
] as const

export type Time = (typeof TIMES)[number]
export type Weather = (typeof WEATHERS)[number]
export type Location = (typeof LOCATIONS)[number]

export type BannerCombo = {
  time: Time
  weather: Weather
  location: Location
  id: string
}

/** Cat activity modes — cycled dynamically while the banner is on screen. */
export type CatBehavior = 'trot' | 'sprint' | 'sniff' | 'passout'

/** Ground Y in the 420×140 banner viewBox — paw pads plant here. */
export const GROUND_Y = 118
