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
import portal.Helper as Helper
import portal.Prihlasovanie as Prihlasovanie
import portal.Subor as Subor
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ConditionType as ConditionType

Prihlasovanie prihlasovanie = new Prihlasovanie()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.selectOptionByLabel(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/2kolo/Select_2kolo'), '2.kolo', false)

'Vyhľadanie prihlášky na SŠ'
WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/button_(nepovinn)_btn-zoradit-podla-predvolene'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/input_Poda priezviska (abecedne - vzostupne_a91dc1'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.waitForJQueryLoad(20)

String priezvisko = WebUI.getText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'))

String meno = WebUI.getText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/button_vedomostn vrtane predmetovch_btn-vyz_863f82'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Vyiada prlohu  ePrihlky/select_(nepovinn)_select-TypPrilohySelect'), 
    '3', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Vyiada prlohu  ePrihlky/textarea_(nepovinn)_textarea-dovodText'), 
    'Príloha SŠ')

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Vyiada prlohu  ePrihlky/button_Sp_btn-odoslat govuk-button govuk-bu_528af4'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/b'), 
    'Žiadosť o doplnenie prílohy bola úspešne odoslaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge red'), 
    'Neúplná')

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/button_Dvod_data-prihlaska-odvolat-ziadost _a29034'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/button_Nie_btn-confirm govuk-button govuk-b_cf1cc7'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge'), 
    'V spracovaní')

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/button_vedomostn vrtane predmetovch_btn-vyz_863f82'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Vyiada prlohu  ePrihlky/select_(nepovinn)_select-TypPrilohySelect'), 
    '3', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Vyiada prlohu  ePrihlky/textarea_(nepovinn)_textarea-dovodText'), 
    'Príloha SŠ 1')

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Vyiada prlohu  ePrihlky/button_Sp_btn-odoslat govuk-button govuk-bu_528af4'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge red'), 
    'Neúplná')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasPouzivatela('ljxikynq7v@dollicons.com', 'w1oXMoeykcdLiib/wAKM5A==', false, GlobalVariable.F2A)

'ZZ doplnenie prílohy na SŠ'
//WebUI.rightClick(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Moje prihlky  ePrihlky/span_warning_panel-text'))

WebUI.verifyTextPresent('Riaditeľ strednej školy požaduje doplnenie príloh. Pridanie prílohy nájdete v stĺpci Akcia.', false)

//nájdenie správnej prihlášky podľa mena dieťaťa
int cislo = 5

while (cislo <= 1000) {
	String xpath = ('//div[' + cislo) + ']/div/div/div/div[2]/span'

	TestObject dynamicObj = new TestObject()

	dynamicObj.addProperty('xpath', ConditionType.EQUALS, xpath)

	if (WebUI.getText(dynamicObj) == meno + " " + priezvisko) {
		break
	}
	
	cislo++
}

//stav prihlášky
TestObject stavPrihlaskyBadge = new TestObject()

stavPrihlaskyBadge.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div[1]/div[3]/div[1]/table[2]/tbody[1]/tr[1]/td[4]/span[1]/span[1]')

WebUI.verifyElementText(stavPrihlaskyBadge, 'Neúplná')

//pridanie prílohy do prihlášky
TestObject pridajPrilohuButton = new TestObject()

pridajPrilohuButton.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div/div[3]/div/table[2]/tbody/tr/td[5]/span/a')

WebUI.click(pridajPrilohuButton)

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Priloi dokumenty  ePrihlky/div_Nahran_govuk-accordion__section-desc-te_b9349f'))

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Priloi dokumenty  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'),
	priloha)

WebUI.waitForJQueryLoad(20)

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Priloi dokumenty  ePrihlky/button_Sp_btn-odoslat govuk-button govuk-bu_528af4'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Priloi dokumenty  ePrihlky/a_Vaa prihlka bude oskoro posden. akujeme z_527046'))

cislo = 5

while (cislo <= 1000) {
	String xpath = ('//div[' + cislo) + ']/div/div/div/div[2]/span'

	TestObject dynamicObj = new TestObject()

	dynamicObj.addProperty('xpath', ConditionType.EQUALS, xpath)

	if (WebUI.getText(dynamicObj) == meno + " " + priezvisko) {
		break
	}
	
	cislo++
}

//stav prihlášky
TestObject stavPrihlaskyBadgeUpdate = new TestObject()

stavPrihlaskyBadgeUpdate.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div[1]/div[3]/div[1]/table[2]/tbody[1]/tr[1]/td[4]/span[1]/span[1]')

WebUI.verifyElementText(stavPrihlaskyBadgeUpdate, 'Doplnená')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.selectOptionByLabel(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/2kolo/Select_2kolo'), '2.kolo', false)

'Kontrola na SŠ'
/**
WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/select_(nepovinn)_select-odborIndexSelect'), 
    'c2a185d7-bb2e-4ffb-b94e-04900a00a698', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/select_(nepovinn)_select-odborIndexSelect'), 
    '69cfc3ec-1e82-4ee0-a3ca-72af42e0f8ea', true)
**/
WebUI.setText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    meno + " " + priezvisko)

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.click(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge_1'), 
    'Doplnená')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/VyziadaniePrilohySS/Page_Podrobnosti prihlky  ePrihlky/div_Doplnen podklady_sprava-item'), 
    0)

prihlasovanie.odhlasPouzivatela()