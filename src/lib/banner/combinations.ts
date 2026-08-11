import { TIMES, WEATHERS, LOCATIONS, type BannerCombo } from './types'

export function allCombos(): BannerCombo[] {
  const out: BannerCombo[] = []
  for (const time of TIMES) {
    for (const weather of WEATHERS) {
      for (const location of LOCATIONS) {
        out.push({
          time,
          weather,
          location,
          id: `${time}-${weather}-${location}`,
        })
      }
    }
  }
  return out
}

export const LABELS = {
  time: { sunrise: 'Sunrise', day: 'Day', sunset: 'Sunset', night: 'Night' },
  weather: { fall: 'Fall', winter: 'Winter', spring: 'Spring', summer: 'Summer' },
  location: {
    city: 'City',
    tokyo: 'Tokyo',
    london: 'London',
    farm: 'Farm',
    forest: 'Forest',
    mountain: 'Mountain',
    beach: 'Beach',
    harbor: 'Harbor',
    desert: 'Desert',
    suburb: 'Suburb',
  },
} as const
