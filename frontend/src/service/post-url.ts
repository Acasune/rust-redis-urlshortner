import { ApiContext } from 'types'
import { fetcher } from 'utils'

export type AddUrl = {
    ret: String
    isLoading: boolean
    isError: boolean
  }

export type AddUrlProps = {
    user_name:string
    url:string
} 
export const addUrl = async (prop:AddUrlProps,context:ApiContext) : Promise<AddUrl> => {
    return await fetcher(`${context.apiRootUrl}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body:  JSON.stringify(prop),
      })
}
