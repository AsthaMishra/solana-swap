'use client'

import { BASIC_PROGRAM_ID as programId, getBasicProgram } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { useMutation, useQuery } from '@tanstack/react-query'

import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

interface Offer{
  id: string,
  token_amount_a_offered: number,
  token_amount_b_wanted: number,
  tokenAMint: string,
  tokenBMint: string
}

export function useBasicProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const program = getBasicProgram(provider)

const accounts = useQuery({
  queryKey: ['swap', 'all',{ cluster }],
  queryFn: () => program.account.offer.all(),
})  

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const make_offer = useMutation<string, Error, Offer>({
    mutationKey: ['swap', 'make_offer', { cluster }],
    mutationFn: ({
      id,
      token_amount_a_offered,
      token_amount_b_wanted,
      tokenAMint,
      tokenBMint
    }) => program.methods.make_offer.rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })

  return {
    program,
    programId,
    getProgramAccount,
    greet,
  }
}
