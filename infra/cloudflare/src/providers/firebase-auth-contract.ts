export interface FirebaseConfig {
  projectId: string;
  issuer: string;
  audience: string;
  jwksUrl: string;
  clockSkewSeconds: number;
  jwksCacheSeconds: number;
}

export interface FirebaseJsonWebKey extends JsonWebKey {
  kid?: string;
}
