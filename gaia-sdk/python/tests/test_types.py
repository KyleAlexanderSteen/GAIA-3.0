from gaia_sdk import GaiaClient, GaiaError
from gaia_sdk.errors import InvalidArgument, NotImplementedCapability
from gaia_sdk.types import AgentSpec, ResourceSpec


def test_intent_is_not_implemented():
    try:
        GaiaClient().intent("hello gaia")
        raise AssertionError("expected NotImplementedCapability")
    except NotImplementedCapability as err:
        assert "nothing was admitted" in str(err)


def test_intent_rejects_empty():
    try:
        GaiaClient().intent("  ")
        raise AssertionError("expected InvalidArgument")
    except InvalidArgument:
        pass


def test_context_and_declare_are_not_implemented():
    try:
        GaiaClient().context("hello")
        raise AssertionError("expected NotImplementedCapability")
    except NotImplementedCapability:
        pass
    try:
        GaiaClient().declare(ResourceSpec(name="cpu"))
        raise AssertionError("expected NotImplementedCapability")
    except NotImplementedCapability:
        pass


def test_invoke_is_not_implemented():
    try:
        GaiaClient().invoke(AgentSpec(agent_id="a", name="researcher"))
        raise AssertionError("expected NotImplementedCapability")
    except NotImplementedCapability as err:
        assert "nothing" in str(err) or "no registered" in str(err)


def test_sign_unimplemented():
    try:
        GaiaClient().sign(b"x")
        raise AssertionError("expected NotImplementedCapability")
    except NotImplementedCapability:
        pass


def test_error_hierarchy():
    assert issubclass(InvalidArgument, GaiaError)
    assert issubclass(NotImplementedCapability, GaiaError)
