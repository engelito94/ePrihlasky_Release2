import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import portal.Prihlasovanie as Prihlasovanie
import portal.Subor as Subor
import groovy.json.JsonSlurper as JsonSlurper
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

Subor subor = new Subor()

def filePath = RunConfiguration.getProjectDir()

//ziskanie domeny cez api
'nájdenie aktuálnej domény'
def getDomainResponse = WS.sendRequest(findTestObject('Object Repository/API mail/GetDomain'))

def jsonDomainResponse = new JsonSlurper().parseText(getDomainResponse.getResponseBodyContent())

String domain = (jsonDomainResponse['hydra:member'])[0].domain

String email = (CustomKeywords.'RandomData.getRandomEmail'() + '@') + domain

String password = CustomKeywords.'RandomData.getRandomPassword'()

//vytvorenie mailu v API
'vytvorenie emailu cez API'
def createResponse = WS.sendRequest(findTestObject('Object Repository/API mail/CreateMail', [('email') : email.toLowerCase()
            , ('password') : password]))

WS.verifyResponseStatusCode(createResponse, 201)

'ziskanie tokenu'
def tokenResponse = WS.sendRequest(findTestObject('Object Repository/API mail/GetToken', [('address') : email.toLowerCase()
            , ('password') : password]))

WS.verifyResponseStatusCode(tokenResponse, 200)

def jsonResponse = new JsonSlurper().parseText(tokenResponse.getResponseBodyContent())

String jwtToken = jsonResponse.token

'UI registrácia'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Prihlsi sa_govuk-button govuk-button__basic'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/h3_prihlste sa_registerLink'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_(nepovinn)_input-email'), 
    email.toLowerCase())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_Zadajte e-mail manulne (nie skoprovan_43a006'), 
    email.toLowerCase())

WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_Min. 8 znakov. Mus obsahova vek psmen_b2b2b7'), 
    'w1oXMoeykcdLiib/wAKM5A==')

WebUI.setEncryptedText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_(nepovinn)_input-zopakujte-heslo'), 
    'w1oXMoeykcdLiib/wAKM5A==')

def udaje = subor.nacitajNahodnyZaznam(filePath + '/Data Files/registracneData.txt')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_(nepovinn)_input-meno'), 
    udaje.meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    udaje.priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_Uvete presne tak, ako je uveden vo va_f508d3'), 
    udaje.rc)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_(nepovinn)_input-cislo-op'), 
    udaje.cislo)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/input_keyboard_arrow_down_suhlas-input'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/button_Toto pole je povinn_btn-zaregistrova_0a3da4'))

WebUI.waitForJQueryLoad(150)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Registrcia  ePrihlky/div_Sksi znova_title'), 
    'Potvrdenie registrácie')

'prečítanie prijatých správ cez API'
def messagesResponse = WS.sendRequest(findTestObject('Object Repository/API mail/GetMessages', [('jwtToken') : jwtToken]))

WS.verifyResponseStatusCode(messagesResponse, 200)

def responseBody = messagesResponse.getResponseBodyContent()

def json = new JsonSlurper().parseText(responseBody)

def pocitadlo = 10

while ((!(json['hydra:member']) || ((json['hydra:member']).size() <= 0)) && (pocitadlo > 0)) {
    WebUI.delay(5)

    messagesResponse = WS.sendRequest(findTestObject('Object Repository/API mail/GetMessages', [('jwtToken') : jwtToken]))

    WS.verifyResponseStatusCode(messagesResponse, 200)

    responseBody = messagesResponse.getResponseBodyContent()

    json = new JsonSlurper().parseText(responseBody)

    pocitadlo--
}

def messageId = (json['hydra:member'])[0].id

'získanie textu konkrétneho mailu'
def messageResponse = WS.sendRequest(findTestObject('Object Repository/API mail/GetMessageById', [('id') : messageId, ('jwtToken') : jwtToken]))

WS.verifyResponseStatusCode(messageResponse, 200)

def jsonMessage = new JsonSlurper().parseText(messageResponse.getResponseBodyContent())

def text = jsonMessage.text

def cleanedText = text.replaceAll('\\s+', '' // odstráni všetky medzery a nové riadky
    )

def url = cleanedText.takeBetween('odkaz[', ']')

if (url) {
    println('Nájdený odkaz: ' + url)
} else {
    println('Odkaz nebol nájdený')
}

'potvrdenie registrácie'
WebUI.navigateToUrl(url)

//WebUI.switchToWindowTitle('Potvrdenie registrácie | ePrihlášky')
WebUI.verifyElementNotVisible(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Potvrdenie registrcie  ePrihlky/span_Prihlsi sa_failure-icon'))

WebUI.verifyElementNotVisible(findTestObject('Object Repository/Zak_test/Release2/Registracia/Page_Potvrdenie registrcie  ePrihlky/h3_Prihlsi sa_failure-heading'))

'prihlasovanie novo zaregistrovaným účtom'
WebUI.delay(80)

prihlasovanie.prihlasPouzivatela(email.toLowerCase(), 'w1oXMoeykcdLiib/wAKM5A==', GlobalVariable.F2A, false)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/span_Menu_inicialy-osoby-header'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/NovePrihlasovanie/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Mj profil_logoutBtn'))

