import { Configuration, HTTPHeaders } from "../generated-api";
import { getBasePath } from "./path";

let configuration: Configuration;
let apiToken: string;

export const setAuthToken = (token: string) => {
  console.log(token);
  apiToken = token;
};

export const getConfiguration = (): Configuration => {
  if (!configuration) {
    const headers: HTTPHeaders = {
      "UNDERWORLD-TOKEN": apiToken,
    };
    configuration = new Configuration({ basePath: getBasePath(), headers });
  }

  return configuration;
};
