/* tslint:disable */
/* eslint-disable */
/**
 * Underworld
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.6.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import {
    GameState,
    GameStateFromJSON,
    GameStateToJSON,
    GeneratedGame,
    GeneratedGameFromJSON,
    GeneratedGameToJSON,
} from '../models';

export interface GameStateRequest {
    gameId: string;
}

export interface RawExportRequest {
    gameId: string;
}

export interface UnlockKnowledgeRequest {
    gameId: string;
}

/**
 * GamesApi - interface
 * 
 * @export
 * @interface GamesApiInterface
 */
export interface GamesApiInterface {
    /**
     * 
     * @summary Get the current state of the game. Will return some inner state and views of the rooms based on the knowledge gained from all players from the game.
     * @param {string} gameId 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof GamesApiInterface
     */
    gameStateRaw(requestParameters: GameStateRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<GameState>>;

    /**
     * Get the current state of the game. Will return some inner state and views of the rooms based on the knowledge gained from all players from the game.
     */
    gameState(requestParameters: GameStateRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<GameState>;

    /**
     * # Example  POST `/games/generate` to generate and save a new game
     * @summary Generate and persist a new game.
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof GamesApiInterface
     */
    generateGameRaw(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<GeneratedGame>>;

    /**
     * # Example  POST `/games/generate` to generate and save a new game
     * Generate and persist a new game.
     */
    generateGame(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<GeneratedGame>;

    /**
     * # Example  Call `/games/ids` to retrieve all of you game ids.
     * @summary Get IDs of all current games
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof GamesApiInterface
     */
    getGameIdsRaw(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<Array<string>>>;

    /**
     * # Example  Call `/games/ids` to retrieve all of you game ids.
     * Get IDs of all current games
     */
    getGameIds(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<Array<string>>;

    /**
     * 
     * @summary Get the current state of the game. This is a raw export and the inner structure is intentionally not documented in the Open API. It matches the structure of the GameState struct inside of the `underworld_core` repository. However, since this is an internal structure to the game, it should not be relied on for any use. The documented one returned in `/state` is less likely to change drastically.
     * @param {string} gameId 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof GamesApiInterface
     */
    rawExportRaw(requestParameters: RawExportRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<any>>;

    /**
     * Get the current state of the game. This is a raw export and the inner structure is intentionally not documented in the Open API. It matches the structure of the GameState struct inside of the `underworld_core` repository. However, since this is an internal structure to the game, it should not be relied on for any use. The documented one returned in `/state` is less likely to change drastically.
     */
    rawExport(requestParameters: RawExportRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<any>;

    /**
     * 
     * @summary Unlock all of the knowledge in the game for all player characters.
     * @param {string} gameId 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof GamesApiInterface
     */
    unlockKnowledgeRaw(requestParameters: UnlockKnowledgeRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<void>>;

    /**
     * Unlock all of the knowledge in the game for all player characters.
     */
    unlockKnowledge(requestParameters: UnlockKnowledgeRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<void>;

}

/**
 * 
 */
export class GamesApi extends runtime.BaseAPI implements GamesApiInterface {

    /**
     * Get the current state of the game. Will return some inner state and views of the rooms based on the knowledge gained from all players from the game.
     */
    async gameStateRaw(requestParameters: GameStateRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<GameState>> {
        if (requestParameters.gameId === null || requestParameters.gameId === undefined) {
            throw new runtime.RequiredError('gameId','Required parameter requestParameters.gameId was null or undefined when calling gameState.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["UNDERWORLD-TOKEN"] = this.configuration.apiKey("UNDERWORLD-TOKEN"); // UnderworldApiKeyAuthorization authentication
        }

        const response = await this.request({
            path: `/games/{game_id}/state`.replace(`{${"game_id"}}`, encodeURIComponent(String(requestParameters.gameId))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GameStateFromJSON(jsonValue));
    }

    /**
     * Get the current state of the game. Will return some inner state and views of the rooms based on the knowledge gained from all players from the game.
     */
    async gameState(requestParameters: GameStateRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<GameState> {
        const response = await this.gameStateRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * # Example  POST `/games/generate` to generate and save a new game
     * Generate and persist a new game.
     */
    async generateGameRaw(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<GeneratedGame>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["UNDERWORLD-TOKEN"] = this.configuration.apiKey("UNDERWORLD-TOKEN"); // UnderworldApiKeyAuthorization authentication
        }

        const response = await this.request({
            path: `/games/generate`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GeneratedGameFromJSON(jsonValue));
    }

    /**
     * # Example  POST `/games/generate` to generate and save a new game
     * Generate and persist a new game.
     */
    async generateGame(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<GeneratedGame> {
        const response = await this.generateGameRaw(initOverrides);
        return await response.value();
    }

    /**
     * # Example  Call `/games/ids` to retrieve all of you game ids.
     * Get IDs of all current games
     */
    async getGameIdsRaw(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<Array<string>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["UNDERWORLD-TOKEN"] = this.configuration.apiKey("UNDERWORLD-TOKEN"); // UnderworldApiKeyAuthorization authentication
        }

        const response = await this.request({
            path: `/games/ids`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse<any>(response);
    }

    /**
     * # Example  Call `/games/ids` to retrieve all of you game ids.
     * Get IDs of all current games
     */
    async getGameIds(initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<Array<string>> {
        const response = await this.getGameIdsRaw(initOverrides);
        return await response.value();
    }

    /**
     * Get the current state of the game. This is a raw export and the inner structure is intentionally not documented in the Open API. It matches the structure of the GameState struct inside of the `underworld_core` repository. However, since this is an internal structure to the game, it should not be relied on for any use. The documented one returned in `/state` is less likely to change drastically.
     */
    async rawExportRaw(requestParameters: RawExportRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<any>> {
        if (requestParameters.gameId === null || requestParameters.gameId === undefined) {
            throw new runtime.RequiredError('gameId','Required parameter requestParameters.gameId was null or undefined when calling rawExport.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["UNDERWORLD-TOKEN"] = this.configuration.apiKey("UNDERWORLD-TOKEN"); // UnderworldApiKeyAuthorization authentication
        }

        const response = await this.request({
            path: `/games/{game_id}/raw_export`.replace(`{${"game_id"}}`, encodeURIComponent(String(requestParameters.gameId))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.TextApiResponse(response) as any;
    }

    /**
     * Get the current state of the game. This is a raw export and the inner structure is intentionally not documented in the Open API. It matches the structure of the GameState struct inside of the `underworld_core` repository. However, since this is an internal structure to the game, it should not be relied on for any use. The documented one returned in `/state` is less likely to change drastically.
     */
    async rawExport(requestParameters: RawExportRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<any> {
        const response = await this.rawExportRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Unlock all of the knowledge in the game for all player characters.
     */
    async unlockKnowledgeRaw(requestParameters: UnlockKnowledgeRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.gameId === null || requestParameters.gameId === undefined) {
            throw new runtime.RequiredError('gameId','Required parameter requestParameters.gameId was null or undefined when calling unlockKnowledge.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["UNDERWORLD-TOKEN"] = this.configuration.apiKey("UNDERWORLD-TOKEN"); // UnderworldApiKeyAuthorization authentication
        }

        const response = await this.request({
            path: `/games/{game_id}/unlock_knowledge`.replace(`{${"game_id"}}`, encodeURIComponent(String(requestParameters.gameId))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Unlock all of the knowledge in the game for all player characters.
     */
    async unlockKnowledge(requestParameters: UnlockKnowledgeRequest, initOverrides?: RequestInit | runtime.InitOverideFunction): Promise<void> {
        await this.unlockKnowledgeRaw(requestParameters, initOverrides);
    }

}
