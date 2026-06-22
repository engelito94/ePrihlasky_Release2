package portal;
import internal.GlobalVariable;
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint;
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase;
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData;
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject;
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject;

import com.kms.katalon.core.annotation.Keyword;
import com.kms.katalon.core.checkpoint.Checkpoint;
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords;
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords;
import com.kms.katalon.core.model.FailureHandling;
import com.kms.katalon.core.testcase.TestCase;
import com.kms.katalon.core.testdata.TestData;
import com.kms.katalon.core.testobject.TestObject;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords;
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords;
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords;

public class Helper {
	String rcToDatumNarodenia(String rc) {
		// Odstráň lomítko
		def cislo = rc.replaceAll(/\//, '')

		// Minimálna dĺžka
		if (!cislo || cislo.length() < 6) {
			throw new IllegalArgumentException("Nesprávny formát rodného čísla: ${rc}")
		}

		// Prvých 6 čísel = [rok%100][mesiac][den]
		def rokKod = cislo.substring(0, 2).toInteger()
		def mesiacKod = cislo.substring(2, 4).toInteger()
		def den = cislo.substring(4, 6).toInteger()

		// Oprava mesiaca pre ženy (-50)
		def mesiac = mesiacKod > 50 ? mesiacKod - 50 : mesiacKod

		// Určenie storočia podľa prvých 2 čísel RC
		def stvrtsto = cislo.substring(0, 2).toInteger()
		def plnyRok
		if (stvrtsto >= 0 && stvrtsto <= 71) {
			plnyRok = 2000 + rokKod  // 21. storočie
		} else {
			plnyRok = 1900 + rokKod  // 20. storočie
		}

		// Formátovanie výstupu
		def datumStr = String.format("%04d-%02d-%02d", plnyRok, mesiac, den)

		try {
			def datum = Date.parse("yyyy-MM-dd", datumStr)
			return datum.format("dd.MM.yyyy")
		} catch (e) {
			throw new IllegalArgumentException("Nedá sa skonvertovať rodné číslo na platný dátum: ${rc}")
		}
	}

	boolean isMuz(String rc) {
		// Odstráň nepotrebné znaky (lomka, medzery, atď.)
		def cislo = rc.replaceAll(/[^0-9]/, '')

		// Minimalná dĺžka rodného čísla (min. prvých 4 čísel)
		if (!cislo || cislo.length() < 4) {
			throw new IllegalArgumentException("Nesprávny formát rodného čísla: ${rc}")
		}

		def mesiac = cislo.substring(2, 4).toInteger()

		// Muž: mesiac 01-12, žena: mesiac 51-62 (po úprave -50 = 01-12)
		return mesiac <= 12
	}

	private static String cleanupCidUrls(String text) {
		if (text == null) return null;
		// vymaže všetky výskyty typu "cid:image001.png@01DCCE4D.74ED1390" / "[cid:...]"
		return text
				.replaceAll("\\[?cid:[^\\s\\]]+\\]?", "")
				.replaceAll("\\s{2,}", " ")   // nahradí viacero medzier jednou
				.trim();
	}
}