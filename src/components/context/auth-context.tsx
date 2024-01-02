import { createContext, useEffect, useState } from "react";
import { Nullable } from "../../types";
import { useNavigate } from "react-router-dom";

type AuthContextType = {
  token: Nullable<string>;
};

const AuthContext = createContext<AuthContextType>({ token: null });

interface Props {
  children: React.ReactNode;
}

export function AuthContextProvider({ children }: Props) {
  const [token, setToken] = useState<Nullable<string>>(null);
  const [shouldShowModal, setShouldShowModal] = useState(true);

  const navigate = useNavigate();

  useEffect(() => {
    const timer = setTimeout(() => {
      if (token !== null) {
        setToken(null);
        setShouldShowModal(true);
      }
    }, 360_000);
    return () => clearTimeout(timer);
  }, [token]);
}
