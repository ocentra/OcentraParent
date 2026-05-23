export const PublicRouteKey = {
  Home: 'home',
  Shop: 'shop',
  CardGamesExplorer: 'card-games-explorer',
  Leaderboard: 'leaderboard',
} as const;

export const PublicRoutePath = {
  [PublicRouteKey.Home]: '/',
  [PublicRouteKey.Shop]: '/shop',
  [PublicRouteKey.CardGamesExplorer]: '/CardGamesExplorer',
  [PublicRouteKey.Leaderboard]: '/leaderboard',
} as const;
