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
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.openqa.selenium.Keys as Keys

// Automat 1 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Automat 1')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'), 
    'agropodnikanie - poľnohospodársky manažment')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 1 - netalent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 1')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'stolár - výroba nábytku a zariadení')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 1 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 1')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'ekonomická teória - zameranie')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 1 - talent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 1')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'reklamná tvorba')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 2 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 2')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'hudba - hra na klarinete')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 2 - netalent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 2')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'výživa a šport')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 2 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 2')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'grafický dizajn')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 2 - talent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 2')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'výtvarné spracúvanie skla-hutnícke tvarovanie skla - hutnícke tvarovanie skla')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 3 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 3')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'paleontológia - paleontológia treťohorných dinosaurov')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 3 - netalent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 3')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'hotelová akadémia')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 3 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 3')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'umelecký čalúnnik a dekoratér')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 3 - talent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 3')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'spev - komorná hudba')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 4 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 4')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'inštalatér')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 4 - netalent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 4')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'technik informačných a telekomunikačných technológií')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 4 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 4')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'obrazová a zvuková tvorba - kamera, zvuk, strih')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 4 - talent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 4')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'animovaná tvorba')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 5 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 5')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'krajčír - pánske odevy')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 5 - netalent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 5')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'kominár')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 5 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 5')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'obrazová a zvuková tvorba - kamera, zvuk, strih')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

// Automat 5 - talent 2
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 5')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_up_govuk-input input-s_853611'),
	'hudba - hra na organe')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_keyboard_arrow_up_govuk-label govuk-c_99700c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))
