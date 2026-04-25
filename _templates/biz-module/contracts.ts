// Shared type template for packages/com/src/contracts.ts.
// Add this block near the bottom of contracts.ts after replacing placeholders.
// Replace __ITEM__ / __item__ with the real business entity name.

export type __ITEM__Record = {
  id: number
  title: string
  status: string
  createdAt: string
  updatedAt: string
}

export type Create__ITEM__Input = {
  title: string
  // Add required fields here.
}

export type Update__ITEM__Input = {
  title?: string
  // Add optional update fields here.
}
