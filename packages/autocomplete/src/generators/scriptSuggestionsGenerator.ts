import logger from "loglevel";
import { executeCommandTimeout } from "@amzn/fig-io-api-bindings-wrappers";
import { runPipingConsoleMethods } from "../utils";
import {
  GeneratorContext,
  haveContextForGenerator,
  runCachedGenerator,
} from "./helpers";

export async function getScriptSuggestions(
  generator: Fig.Generator,
  context: GeneratorContext,
  defaultTimeout: number,
): Promise<Fig.Suggestion[]> {
  const { script, postProcess, splitOn } = generator;
  if (!script) {
    return [];
  }

  if (!haveContextForGenerator(context)) {
    logger.info("Don't have context for custom generator");
    return [];
  }

  try {
    const { isDangerous, tokenArray, currentWorkingDirectory } = context;
    // A script can either be a string or a function that returns a string.
    // If the script is a function, run it, and get the output string.
    const commandToRun =
      script && typeof script === "function"
        ? runPipingConsoleMethods(() => script(tokenArray))
        : script;

    if (!commandToRun) {
      return [];
    }

    let executeCommandInput: Fig.ExecuteCommandInput;
    if (Array.isArray(commandToRun)) {
      executeCommandInput = {
        command: commandToRun[0],
        args: commandToRun.slice(1),
        cwd: currentWorkingDirectory,
      };
    } else {
      executeCommandInput = {
        cwd: currentWorkingDirectory,
        ...commandToRun,
      };
    }

    // Use the longest duration timeout
    const timeout = Math.max(
      defaultTimeout,
      generator.scriptTimeout ?? 0,
      executeCommandInput.timeout ?? 0,
    );

    const { stdout } = await runCachedGenerator(
      generator,
      context,
      () => executeCommandTimeout(executeCommandInput, timeout),
      generator.cache?.cacheKey ?? JSON.stringify(executeCommandInput),
    );

    let result: Array<Fig.Suggestion | string> = [];

    // If we have a splitOn function
    if (splitOn) {
      result = stdout.trim() === "" ? [] : stdout.trim().split(splitOn);
    } else if (postProcess) {
      // If we have a post process function
      // The function takes one input and outputs an array
      runPipingConsoleMethods(() => {
        result = postProcess(stdout, tokenArray);
      });
      result = result.filter(
        (item) => item && (typeof item === "string" || !!item.name),
      );
    }

    // Generator can either output an array of strings or an array of suggestion objects.
    return result.map((item) =>
      typeof item === "string"
        ? { type: "arg", name: item, insertValue: item, isDangerous }
        : { ...item, type: item.type || "arg" },
    );
  } catch (e) {
    logger.error("we had an error with the script generator", e);
    return [];
  }
}