describe('rLoign modal interaction', () => {
  beforeEach(() => {
    cy.visit('/')
    cy.contains('login with rLogin').click()
  })

  it('opens the popup', () => {
    cy.contains('Connect your wallet')
  })

  it('closes the popup when clicking outside of it', () => {
    cy.get('body').click('topRight')
    cy.get('.rlogin-modal-lightbox').should('be.not.visible')
  })

  it('closes the popup when clicking the close X', () => {
    cy.get('.rlogin-modal-close-button').click()
    cy.get('.rlogin-modal-lightbox').should('be.not.visible')
  })

  it('shows a QR code for WalletConnect', { defaultCommandTimeout: 10000 }, () => {
    cy.contains('WalletConnect').click()
    cy.get('wcm-modal')
  })
})
