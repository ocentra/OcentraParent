export function useLocation() {
  return {
    pathname: globalThis.location?.pathname ?? '/',
  };
}

export function useNavigate() {
  return (path: string) => {
    globalThis.location.href = path;
  };
}
