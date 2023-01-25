import { ApiContext } from 'types'
import { fetcher } from 'utils'

export type DelUrl = {
    hash: String
    isLoading: boolean
    isError: boolean
  }

export type DelUrlProps = {
    hash:string
} 
export const delUrl = async ({hash}:DelUrlProps,context:ApiContext) : Promise<DelUrl> => {
    return await fetcher(`${context.apiRootUrl.replace(/\/$/g, '')}/${hash}`, {
        method: 'DELETE'
    })
}
