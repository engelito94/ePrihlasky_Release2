import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.time.LocalDate as LocalDate
import java.time.LocalDateTime as LocalDateTime
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
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

def filePath = RunConfiguration.getProjectDir()

def pozvanka = filePath + '/Data Files/PozvánkaPredloha.pdf'

def body = filePath + '/Data Files/BodyPredloha.pdf'

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/a_Prihlky_govuk-header__link'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/h1_Odoslan sprvy o bodoch_govuk-heading-m'), 
    'Pozvánky na prijímacie skúšky')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/span_Nvod na pouvanie_material-icons govuk-_38ec53'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/strong'), 
    'Odoslanie správy o plnom počte bodov')

WebUI.verifyTextPresent('Pre žiakov, ktorí nerobia prijímacie skúšky kvôli dobrým výsledkom na Testovaní 9 (T9), môžete hromadne odoslať správu o plnom počte bodov na e-mail zákonného zástupcu žiaka.', 
    false)

WebUI.verifyTextPresent('Označte vybraných uchádzačov, kliknite na tlačidlo "Vykonať akciu" a vyberte "Odoslať správu o plnom počte bodov".', 
    false)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/strong_1'), 
    'Hromadná pozvánka na prijímacie skúšky')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/p_1'), 
    'Pre žiakov, ktorí robia prijímacie skúšky, môžete hromadne nastaviť dátum, čas a miestnosť. Označte vybraných uchádzačov, kliknite na tlačidlo „Nastavenie termínu skúšky“, zadajte požadované údaje a kliknite na tlačidlo "Uložiť". Následne kliknite na tlačidlo "Vykonať akciu" a vyberte "Odoslať pozvánky". Skontrolujte alebo upravte predmet a textové pole, taktiež môžete pripojiť prílohu. Kliknite na tlačidlo "Odoslať".')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/strong_2'), 
    'Úprava údajov pre jednotlivých žiakov a náhradný termín skúšky')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/p_2'), 
    'Pomocou tlačidla „Vybrať“, v možnosti „Upraviť“ môžete zmeniť vybrané hodnoty pre jednotlivých žiakov. V tejto časti viete označiť termín ako náhradný pre konkrétneho žiaka.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/strong_3'), 
    'Stiahnutie komunikácie vo formáte PDF')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/p_3'), 
    'Pozvánky alebo správy o počte bodov si môžete jednotlivo stiahnuť vo formáte PDF na odoslanie poštou kliknutím na tlačidlo „Vybrať“ a „Stiahnuť PDF“. Po odoslaní pomocou tlačidla „Vybrať“ v možnosti „Upraviť“ zmeňte stav vybranej komunikácie na „Odoslaná“ a nezabudnite „Uložiť“.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/strong_4'), 
    'Stav komunikácie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/li'), 
    'Odoslaná - prečítaná alebo neprečítaná v centre oznámení')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/li_1'), 
    'Neodoslaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/li_2'), 
    'Chyba doručenie - odošlite znova pomocou tlačidla „Odoslať znova“.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/strong_5'), 
    'Export tabuľky')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/p_4'), 
    'Tabuľku môžete exportovať vo vybraných formátoch stlačením tlačidla „Export“.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_iakovi nebola odoslan pozvnka ani spr_cba48a'), 
    'Tomáša')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_iakovi nebola odoslan pozvnka ani sp_ab1a3d'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Akcia_govuk-button govuk-button--sec_e3fab9'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/a'))

'úprava'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Filtrova_title'), 
    'Upraviť informácie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/h4_Upravi informcie_prij_edit_meno'), 
    'Balážová Tomáša')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/span_Balov Toma_prij_edit_detaily'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_no_prij__edit_prijimacie_skusky_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-edit govuk-button govuk-but_6cb5b0'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Komunikcia_name-label'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Nie_data-prijimacka-termin badge'), 
    '1. termín (1.kolo)')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Akcia_govuk-button govuk-button--sec_e3fab9'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/a'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_prij__edit_prijimacie_skus_c5a786'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_datumDen'), 
    '11')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_datumMesiac'), 
    '11')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_datumRok'), 
    '2026')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_casHodina'), 
    '11')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_input-prij__edit_casMinuta'), 
    '00')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-edit govuk-button govuk-but_6cb5b0'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_grey-label'), 
    '11.11.2026')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Nhradn termn_grey-label_1'), 
    '11:00')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Komunikcia_name-label_1'), 
    'Áno')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Akcia_govuk-button govuk-button--sec_e3fab9'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/a'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/input_(nepovinn)_prij__edit_nahradny_termin_8558c9'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/button_Zrui_btn-edit govuk-button govuk-but_6cb5b0'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_no_data-prijimacka-nahradny-termin badge linen'), 
    'Náhradný termín')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Prijimacky/vizuálneKontroly/Page_Prihlky a rozhodnutia  ePrihlky/div_Komunikcia_name-label_1'), 
    'Áno')

prihlasovanie.odhlasPouzivatela()

