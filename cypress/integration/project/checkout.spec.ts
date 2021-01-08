import { ipcStub } from "../../support";
import * as commands from "../../support/commands";

context("project checkout", () => {
  const withWorkspaceStub = callback => {
    cy.exec("pwd").then(result => {
      const pwd = result.stdout;
      const checkoutPath = `${pwd}/cypress/workspace/checkout`;

      // Clean up any left-overs from previous failed tests.
      cy.exec(`rm -rf ${checkoutPath}`);
      cy.exec(`mkdir -p ${checkoutPath}`);

      ipcStub.getStubs().then(stubs => {
        stubs.IPC_DIALOG_SHOWOPENDIALOG.returns(checkoutPath);
        stubs.IPC_OPEN_PATH.returns("");
      });

      callback(checkoutPath);

      // Clean up the cypress workspace.
      cy.exec(`rm -rf ${checkoutPath}`);
    });
  };

  beforeEach(() => {
    commands.resetProxyState();
    commands.onboardUser();
    commands.createProjectWithFixture();
    cy.visit("./public/index.html");
  });

  context("project checkout", () => {
    context("git remote helper setup hints", () => {
      it("shows hints on how to set up the remote helper", () => {
        // The hint is visible in the project checkout modal.
        commands.pick("project-list-entry-platinum").click();
        commands.pick("checkout-modal-toggle").click();
        commands.pick("remote-helper-hint").should("be.visible");
        commands.pick("profile").click();

        // The hint is visible in the project creation modal.
        commands.pick("new-project-button").click();
        commands.pick("remote-helper-hint").should("be.visible");

        // Dismiss the hint.
        commands.pick("close-hint-button").click();
        commands.pick("remote-helper-hint").should("not.exist");
        commands.pick("cancel-button").click();

        // Hint is still hidden when re-entering project creation
        commands.pick("new-project-button").click();
        commands.pick("remote-helper-hint").should("not.exist");
        commands.pick("cancel-button").click();

        // The hint is also hidden in the project creation modal.
        commands.pick("new-project-button").click();
        commands.pick("remote-helper-hint").should("not.exist");
      });
    });

    context("happy path", () => {
      it("checks out the project into a working directory", () => {
        commands.pick("project-list-entry-platinum").click();
        commands.pick("checkout-modal-toggle").click();

        withWorkspaceStub(checkoutPath => {
          commands.pick("choose-path-button").click();
          // Make sure UI has time to update path value from stub,
          // prevents this spec from failing on CI.
          cy.wait(500);

          // Make sure mock is set up correctly.
          ipcStub.getStubs().then(stubs => {
            expect(stubs.IPC_OPEN_PATH.called).to.be.false;
          });

          // Perform the checkout.
          commands.pick("checkout-button").click();

          // Notification should contain the full path to the working directory.
          commands
            .pick("notification")
            .contains("platinum checked out to")
            .should("exist");
          commands
            .pick("notification")
            .contains("cypress/workspace/checkout/platinum")
            .should("exist");
          commands.pick("notification").contains("Open folder").should("exist");
          commands.pick("notification").contains("Open folder").click();

          // Make sure we do the electron call for opening the folder in the OS
          // file browser.
          ipcStub.getStubs().then(stubs => {
            expect(stubs.IPC_OPEN_PATH.called).to.be.true;
          });

          // Make sure the notification gets closed after we open the folder in
          // the OS file browser.
          cy.contains("platinum checked out to").should("not.exist");

          // Check that the working directory has the rad remote.
          cy.exec(`git -C ${checkoutPath}/platinum remote show`).then(
            result => {
              expect(result.stdout).to.equal(`rad`);
            }
          );
        });
      });
    });
  });
});
