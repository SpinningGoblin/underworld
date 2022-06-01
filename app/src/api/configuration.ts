import { Configuration } from "../generated-api";
import { getBasePath } from "./path";

let configuration: Configuration;

export const getConfiguration = (): Configuration => {
  if (!configuration) {
    configuration = new Configuration({ basePath: getBasePath() });
  }

  return configuration;
};
