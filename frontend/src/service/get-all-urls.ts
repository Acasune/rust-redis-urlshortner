import useSWR from 'swr'
import { Data } from 'types'
import { fetcher } from 'utils'
export type UseGetAllUrls = {
  ret: [{ hashed: string; url: string }]
  isLoading: boolean
  isError: boolean
}

export const useGetAllUrls = (): UseGetAllUrls => {
  const path: string = process.env.API_BASE_URL || 'http://localhost:8080/'

  const { data, error } = useSWR<Data>(path, fetcher)

  return {
    ret: data ? data.urls : [{ hashed: '', url: '' }],
    isLoading: !error && !data,
    isError: !!error,
  }
}
