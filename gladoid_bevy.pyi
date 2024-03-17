from typing import overload

class Player:
    id: int
    name: str

class GladoidGameWorld:
    @overload
    def insert_action(self, action_id: int, player_id: int) -> None: ...
    @overload
    def insert_action(self, action_id: int, weapon_id: int) -> None: ...
    @overload
    def insert_action(self, action_id: int) -> None: ...
    def tick(self) -> None:
        """
        Run every schedules, change self.state accordingly
        """

    def check_need_action(self) -> bool:
        """
        Check if an input is required from any Player
        """

    def check_need_action_from(self) -> Player | None:
        """
        Check if an input is required. This also returns the player that the input must come from.
        """

    def get_players(self) -> list[Player]: ...
    def get_game_messages(self) -> list[str]:
        """
        Returns the game messages since the last `self.tick`
        """

def create_world() -> GladoidGameWorld: ...
